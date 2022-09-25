use bytes::{BufMut, Bytes, BytesMut};
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::task::JoinHandle;
use wasmrs_host::{
    GuestExports, Handler, ModuleState, ProviderCallContext, WasiParams, WebAssemblyEngineProvider,
};
use wasmrs_rsocket::fragmentation::Splitter;
use wasmrs_rsocket::{runtime, Flux, Frame, Payload, PayloadFrame, RequestResponse};
use wasmtime::{AsContextMut, Engine, Instance, Linker, Module, Store, TypedFunc};

use super::Result;
use crate::errors::Error;
use crate::store::{new_store, WasmRsStore};
use crate::wasmrs_wasmtime::{self, write_bytes_to_memory};

/// A wasmRS engine provider that encapsulates the Wasmtime WebAssembly runtime
#[allow(missing_debug_implementations)]
pub struct WasmtimeEngineProvider {
    module: Module,
    engine: Arc<Engine>,
    linker: Linker<WasmRsStore>,
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

impl Clone for WasmtimeEngineProvider {
    fn clone(&self) -> Self {
        let engine = self.engine.clone();

        let mut new = Self {
            module: self.module.clone(),
            wasi_params: self.wasi_params.clone(),
            engine,
            linker: self.linker.clone(),
            epoch_deadlines: self.epoch_deadlines,
        };
        new.init().unwrap();
        new
    }
}

impl WasmtimeEngineProvider {
    /// Creates a new instance of a [WasmtimeEngineProvider].
    pub fn new(buf: &[u8], wasi: Option<WasiParams>) -> Result<WasmtimeEngineProvider> {
        let engine = Engine::default();
        Self::new_with_engine(buf, engine, wasi)
    }

    /// Creates a new instance of a [WasmtimeEngineProvider] with caching enabled.
    pub fn new_with_cache(
        buf: &[u8],
        wasi: Option<WasiParams>,
        cache_path: Option<&std::path::Path>,
    ) -> Result<WasmtimeEngineProvider> {
        let mut config = wasmtime::Config::new();
        config.strategy(wasmtime::Strategy::Cranelift);
        if let Some(cache) = cache_path {
            config.cache_config_load(cache)?;
        } else if let Err(e) = config.cache_config_load_default() {
            warn!("Wasmtime cache configuration not found ({}). Repeated loads will speed up significantly with a cache configuration. See https://docs.wasmtime.dev/cli-cache.html for more information.",e);
        }
        config.wasm_reference_types(false);
        let engine = Engine::new(&config)?;
        Self::new_with_engine(buf, engine, wasi)
    }

    /// Creates a new instance of a [WasmtimeEngineProvider] from a separately created [wasmtime::Engine].
    pub fn new_with_engine(
        buf: &[u8],
        engine: Engine,
        wasi_params: Option<WasiParams>,
    ) -> Result<Self> {
        let module = Module::new(&engine, buf)?;

        let mut linker: Linker<WasmRsStore> = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s.wasi_ctx.as_mut().unwrap()).unwrap();

        Ok(WasmtimeEngineProvider {
            module,
            engine: Arc::new(engine),
            wasi_params,
            linker,
            epoch_deadlines: None,
        })
    }
}

impl WebAssemblyEngineProvider for WasmtimeEngineProvider {
    fn new_context(
        &self,
        state: Arc<ModuleState>,
    ) -> std::result::Result<
        Arc<Mutex<(dyn ProviderCallContext + Send + Sync + 'static)>>,
        wasmrs_host::errors::Error,
    > {
        // TODO: this is not cheap. Make it faster.
        let store = new_store(&self.wasi_params, &self.engine)
            .map_err(|e| wasmrs_host::errors::Error::NewContext(e.to_string()))?;

        Ok(Arc::new(Mutex::new(
            WasmtimeCallContext::new(state, self.linker.clone(), &self.module, store)
                .map_err(|e| wasmrs_host::errors::Error::InitFailed(e.to_string()))?,
        )))
    }
}

struct WasmtimeCallContext {
    store: Store<WasmRsStore>,
    instance: Instance,
    state: Arc<ModuleState>,
}

impl WasmtimeCallContext {
    pub(crate) fn new(
        state: Arc<ModuleState>,
        mut linker: Linker<WasmRsStore>,
        module: &Module,
        mut store: Store<WasmRsStore>,
    ) -> Result<Self> {
        // THIS IS EXPENSIVE! DO IT ELSEWHERE.
        wasmrs_wasmtime::add_to_linker(&mut linker, &state)?;
        let instance = linker.instantiate(&mut store, module)?;

        let func = instance
            .get_typed_func::<(i32), (), _>(&mut store, GuestExports::Send.as_ref())
            .map_err(|_| crate::errors::Error::GuestSend)?;
        let store = store;

        Ok(Self {
            state,
            instance,
            store,
        })
    }

    pub(crate) fn link(linker: &mut Linker<WasmRsStore>, state: &Arc<ModuleState>) -> Result<()> {
        wasmrs_wasmtime::add_to_linker(linker, state)?;
        Ok(())
    }
}

impl wasmrs_rsocket::FrameWriter for WasmtimeCallContext {
    /// Request-Response interaction model of RSocket.
    fn write_frame(&mut self, stream_id: u32, req: Frame) -> wasmrs_rsocket::Result<()> {
        let (tx, rx) = tokio::sync::oneshot::channel::<wasmrs_rsocket::Result<Option<Payload>>>();
        let sid = stream_id;
        let state = self.state.clone();
        let bytes = req.encode();

        let func: TypedFunc<i32, ()> = self
            .instance
            .get_typed_func(&mut self.store, GuestExports::Send.as_ref())
            .map_err(|_| crate::errors::Error::GuestSend)?;
        let mem = self.instance.get_memory(&mut self.store, "memory").unwrap();

        let read_pos = self.state.get_guest_buffer_pos();
        let buffer_len_bytes = (bytes.len() as u32).to_be_bytes();
        let mut buffer = BytesMut::with_capacity(buffer_len_bytes.len() + bytes.len());
        buffer.put(buffer_len_bytes.as_slice());
        buffer.put(bytes);

        let written = write_bytes_to_memory(
            &mut self.store,
            mem,
            &buffer,
            read_pos,
            self.state.get_guest_buffer_start(),
            self.state.get_guest_buffer_size(),
        );
        self.state.update_guest_buffer_pos(written);

        let _call = func.call(&mut self.store, read_pos as i32);

        Ok(())
    }
}

impl ProviderCallContext for WasmtimeCallContext {
    fn init(&mut self) -> std::result::Result<(), wasmrs_host::errors::Error> {
        let init: TypedFunc<(u32, u32, u32), ()> = self
            .instance
            .get_typed_func(&mut self.store, GuestExports::Init.as_ref())
            .map_err(|e| wasmrs_host::errors::Error::InitFailed(Error::GuestInit.to_string()))?;
        init.call(&mut self.store, (1024, 1024, 128));
        Ok(())
    }
}
