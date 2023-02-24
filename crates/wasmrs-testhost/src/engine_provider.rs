use std::sync::Arc;

use bytes::{BufMut, BytesMut};
use wasmrs::{Frame, OperationList, WasmSocket};
use wasmrs_host::{CallbackProvider, GuestExports, ProviderCallContext, WasiParams};
use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

use super::Result;
use crate::errors::Error;
use crate::memory::write_bytes_to_memory;
use crate::store::{new_store, ProviderStore};
use crate::wasmrs_wasmtime::{self};

/// A wasmRS engine provider that encapsulates the Wasmtime WebAssembly runtime
#[allow(missing_debug_implementations)]
pub struct WasmtimeEngineProvider {
  module: Module,
  engine: Arc<Engine>,
  linker: Linker<ProviderStore>,
  wasi_params: Option<WasiParams>,
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
  pub(crate) fn new_with_engine(buf: &[u8], engine: Engine, wasi_params: Option<WasiParams>) -> Result<Self> {
    let module = Module::new(&engine, buf).map_err(Error::Module)?;

    let mut linker: Linker<ProviderStore> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s.wasi_ctx.as_mut().unwrap()).unwrap();

    Ok(WasmtimeEngineProvider {
      module,
      engine: Arc::new(engine),
      wasi_params,
      linker,
      epoch_deadlines: None,
    })
  }

  /// Create a new call context
  pub fn new_context(
    &self,
    socket: Arc<WasmSocket>,
  ) -> std::result::Result<WasmtimeCallContext, wasmrs_host::errors::Error> {
    let store = new_store(&self.wasi_params, socket, &self.engine)
      .map_err(|e| wasmrs_host::errors::Error::NewContext(e.to_string()))?;

    WasmtimeCallContext::new(self.linker.clone(), &self.module, store)
      .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.to_string()))
  }
}

/// Raw Wasmtime call context
#[allow(missing_debug_implementations)]
pub struct WasmtimeCallContext {
  guest_send: TypedFunc<i32, ()>,
  memory: Memory,
  store: Store<ProviderStore>,
  instance: Instance,
}

impl WasmtimeCallContext {
  pub(crate) fn new(
    mut linker: Linker<ProviderStore>,
    module: &Module,
    mut store: Store<ProviderStore>,
  ) -> Result<Self> {
    wasmrs_wasmtime::add_to_linker(&mut linker)?;
    let instance = linker.instantiate(&mut store, module).map_err(Error::Linker)?;

    let guest_send = instance
      .get_typed_func::<i32, ()>(&mut store, GuestExports::Send.as_ref())
      .map_err(|_| crate::errors::Error::GuestSend)?;
    let memory = instance.get_memory(&mut store, "memory").unwrap();

    Ok(Self {
      guest_send,
      memory,
      instance,
      store,
    })
  }

  /// Send a raw wasmrs frame to the WebAssembly module
  pub fn send_frame(&mut self, req: Frame) -> std::result::Result<(), wasmrs::Error> {
    let bytes = req.encode();
    trace!(?bytes, "writing frame");

    let buffer_len_bytes = wasmrs::util::to_u24_bytes(bytes.len() as u32);
    let mut buffer = BytesMut::with_capacity(buffer_len_bytes.len() + bytes.len());
    buffer.put(buffer_len_bytes);
    buffer.put(bytes);

    let start = self.store.data().guest_buffer.get_start();
    let len = self.store.data().guest_buffer.get_size();

    let written = write_bytes_to_memory(&mut self.store, self.memory, &buffer, start, len);

    let _call = self.guest_send.call(&mut self.store, written as i32);

    Ok(())
  }

  /// Run wasmrs init function.
  pub fn run_init(&mut self) -> std::result::Result<(), wasmrs_host::errors::Error> {
    if let Ok(start) = self
      .instance
      .get_typed_func(&mut self.store, GuestExports::Start.as_ref())
    {
      trace!("Calling tinygo _start method");
      start
        .call(&mut self.store, ())
        .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.to_string()))?;
    }

    let init: TypedFunc<(u32, u32, u32), ()> = self
      .instance
      .get_typed_func(&mut self.store, GuestExports::Init.as_ref())
      .map_err(|_e| wasmrs_host::errors::Error::InitFailed(Error::GuestInit.to_string()))?;
    init
      .call(&mut self.store, (1024, 1024, 128))
      .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.to_string()))?;

    if let Ok(oplist) = self
      .instance
      .get_typed_func::<(), ()>(&mut self.store, GuestExports::OpListRequest.as_ref())
    {
      trace!("Calling operation list");
      oplist.call(&mut self.store, ()).unwrap();
    }
    Ok(())
  }
}

impl wasmrs::ModuleHost for WasmtimeCallContext {
  /// Request-Response interaction model of RSocket.
  fn write_frame(&mut self, req: Frame) -> std::result::Result<(), wasmrs::Error> {
    self.send_frame(req)
  }

  fn get_export(&self, namespace: &str, operation: &str) -> std::result::Result<u32, wasmrs::Error> {
    self
      .store
      .data()
      .get_export(namespace, operation)
      .map_err(|e| wasmrs::Error::OpList(e.to_string()))
  }

  fn get_import(&self, namespace: &str, operation: &str) -> std::result::Result<u32, wasmrs::Error> {
    self
      .store
      .data()
      .get_import(namespace, operation)
      .map_err(|e| wasmrs::Error::OpList(e.to_string()))
  }

  fn get_operation_list(&mut self) -> OperationList {
    self.store.data().op_list.lock().clone()
  }
}

impl ProviderCallContext for WasmtimeCallContext {
  fn init(&mut self) -> std::result::Result<(), wasmrs_host::errors::Error> {
    self.run_init()
  }
}
