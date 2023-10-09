use std::sync::Arc;

use bytes::{BufMut, BytesMut};
use tokio::sync::Mutex;
use wasi_common::WasiCtx;
use wasmrs::{Frame, OperationList, WasmSocket};
use wasmrs_host::{EngineProvider, GuestExports, HostServer, ProviderCallContext, SharedContext};
use wasmtime::{AsContextMut, Engine, Linker, Memory, Module, Store, TypedFunc};

use super::Result;
use crate::errors::Error;
use crate::memory::write_bytes_to_memory;
use crate::store::{new_store, ProviderStore};
use crate::wasmrs_wasmtime::{self};

/// A wasmRS engine provider that encapsulates the Wasmtime WebAssembly runtime
#[allow(missing_debug_implementations)]
pub struct WasmtimeEngineProvider {
  module: Module,
  engine: Engine,
  linker: Linker<ProviderStore<HostServer>>,
  wasi_ctx: Option<WasiCtx>,
  pub(crate) epoch_deadlines: Option<EpochDeadlines>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct EpochDeadlines {
  /// Deadline for wasmRS initialization code. Expressed in number of epoch ticks
  #[allow(dead_code)]
  pub(crate) wasmrs_init: u64,

  /// Deadline for user-defined wasmRS function computation. Expressed in number of epoch ticks
  #[allow(dead_code)]
  pub(crate) wasmrs_func: u64,
}

impl WasmtimeEngineProvider {
  /// Creates a new instance of a [WasmtimeEngineProvider] from a separately created [wasmtime::Engine].
  pub(crate) fn new_with_engine(module: Module, engine: Engine, wasi_ctx: Option<WasiCtx>) -> Result<Self> {
    let mut linker: Linker<ProviderStore<HostServer>> = Linker::new(&engine);

    if wasi_ctx.is_some() {
      wasmtime_wasi::add_to_linker(&mut linker, |s| s.wasi_ctx.as_mut().unwrap()).unwrap();
    }

    Ok(WasmtimeEngineProvider {
      module,
      engine,
      wasi_ctx,
      linker,
      epoch_deadlines: None,
    })
  }
}

#[async_trait::async_trait]
impl EngineProvider for WasmtimeEngineProvider {
  async fn new_context(
    &self,
    socket: Arc<WasmSocket<HostServer>>,
  ) -> std::result::Result<SharedContext, wasmrs_host::errors::Error> {
    let store = new_store(self.wasi_ctx.clone(), socket, &self.engine)
      .map_err(|e| wasmrs_host::errors::Error::NewContext(e.to_string()))?;

    let context = SharedContext::new(
      WasmtimeCallContext::new(self.linker.clone(), &self.module, store)
        .await
        .map_err(|e| wasmrs_host::errors::Error::InitFailed(Box::new(e)))?,
    );

    Ok(context)
  }
}

#[derive(PartialEq, Debug)]
enum Version {
  V0,
  V1,
}

struct Imports {
  start: Option<TypedFunc<(), ()>>,
  guest_init: TypedFunc<(u32, u32, u32), ()>,
  op_list: Option<TypedFunc<(), ()>>,
  guest_send: TypedFunc<i32, ()>,
  version: Version,
}

struct WasmtimeCallContext {
  memory: Memory,
  store: Mutex<Store<ProviderStore<HostServer>>>,
  imports: Imports,
  op_list: parking_lot::Mutex<OperationList>,
}

impl WasmtimeCallContext {
  pub(crate) async fn new(
    mut linker: Linker<ProviderStore<HostServer>>,
    module: &Module,
    mut store: Store<ProviderStore<HostServer>>,
  ) -> Result<Self> {
    wasmrs_wasmtime::add_to_linker(&mut linker)?;
    let instance = linker
      .instantiate_async(&mut store, module)
      .await
      .map_err(Error::Linker)?;

    let guest_send = instance
      .get_typed_func::<i32, ()>(&mut store, GuestExports::Send.as_ref())
      .map_err(|_| crate::errors::Error::GuestSend)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    let version = instance
      .get_typed_func::<(), ()>(&mut store, GuestExports::Version1.as_ref())
      .map_or(Version::V0, |_| Version::V1);

    let imports = Imports {
      version,
      start: instance.get_typed_func(&mut store, GuestExports::Start.as_ref()).ok(),
      guest_init: instance
        .get_typed_func(&mut store, GuestExports::Init.as_ref())
        .map_err(|_e| Error::GuestInit)?,
      op_list: instance
        .get_typed_func::<(), ()>(&mut store, GuestExports::OpListRequest.as_ref())
        .ok(),
      guest_send,
    };

    Ok(Self {
      memory,
      store: Mutex::new(store),
      imports,
      op_list: parking_lot::Mutex::new(OperationList::default()),
    })
  }
}

#[async_trait::async_trait]
impl wasmrs::ModuleHost for WasmtimeCallContext {
  /// Request-Response interaction model of RSocket.
  async fn write_frame(&self, mut req: Frame) -> std::result::Result<(), wasmrs::Error> {
    let bytes = if self.imports.version == Version::V0 {
      req.make_v0_metadata();
      req.encode()
    } else {
      req.encode()
    };
    trace!(?bytes, "writing frame");

    let buffer_len_bytes = wasmrs::util::to_u24_bytes(bytes.len() as u32);
    let mut buffer = BytesMut::with_capacity(buffer_len_bytes.len() + bytes.len());
    buffer.put(buffer_len_bytes);
    buffer.put(bytes);

    let mut store = self.store.lock().await;

    let start = store.data().guest_buffer.get_start();
    let len = store.data().guest_buffer.get_size();

    let written = write_bytes_to_memory(store.as_context_mut(), self.memory, &buffer, start, len);

    self
      .imports
      .guest_send
      .call_async(store.as_context_mut(), written as i32)
      .await
      .map_err(|e| wasmrs::Error::GuestCall(e.to_string()))?;

    Ok(())
  }

  async fn on_error(&self, stream_id: u32) -> std::result::Result<(), wasmrs::Error> {
    let mut lock = self.store.lock().await;
    let data = lock.data_mut();
    if let Err(e) = data.socket.process_once(Frame::new_cancel(stream_id)) {
      error!("error processing cancel for stream id {}, {}", stream_id, e);
    };
    Ok(())
  }

  fn get_import(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.op_list.lock().get_import(namespace, operation)
  }

  fn get_export(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.op_list.lock().get_export(namespace, operation)
  }

  fn get_operation_list(&self) -> OperationList {
    self.op_list.lock().clone()
  }
}

#[async_trait::async_trait]
impl ProviderCallContext for WasmtimeCallContext {
  async fn init(
    &self,
    host_buffer_size: u32,
    guest_buffer_size: u32,
  ) -> std::result::Result<(), wasmrs_host::errors::Error> {
    let mut store = self.store.lock().await;

    if let Some(start) = &self.imports.start {
      start
        .call_async(store.as_context_mut(), ())
        .await
        .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.into()))?;
    }

    self
      .imports
      .guest_init
      .call_async(store.as_context_mut(), (host_buffer_size, guest_buffer_size, 128))
      .await
      .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.into()))?;

    store.data().guest_buffer.update_size(guest_buffer_size);
    store.data().host_buffer.update_size(host_buffer_size);

    if let Some(oplist) = self.imports.op_list {
      trace!("calling operation list");
      oplist
        .call_async(store.as_context_mut(), ())
        .await
        .map_err(|e| wasmrs_host::errors::Error::OpList(e.to_string()))?;

      *self.op_list.lock() = store.data().op_list.clone();
    }

    Ok(())
  }
}
