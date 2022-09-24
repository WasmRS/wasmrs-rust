pub(crate) mod modulestate;
pub(crate) mod traits;

use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_core::Stream;
use rxrust::prelude::*;
use wasmrs_rsocket::frames::{Payload, RequestPayload};
use wasmrs_rsocket::{Frame, Metadata};

use self::modulestate::{ModuleState, ShareableStream};
use self::traits::WebAssemblyEngineProvider;
use crate::{
    errors, AsyncHostCallback, HostCallback, Invocation, OutgoingStream, ProviderCallContext,
};

static GLOBAL_MODULE_COUNT: AtomicU64 = AtomicU64::new(1);
static GLOBAL_CONTEXT_COUNT: AtomicU64 = AtomicU64::new(1);

type Result<T> = std::result::Result<T, crate::errors::Error>;

/// A WebAssembly host runtime for wasmRS-compliant modules
///
/// Use an instance of this struct to provide a means of invoking procedure calls by
/// specifying an operation name and a set of bytes representing the opaque operation payload.
/// `WasmRsHost` makes no assumptions about the contents or format of either the payload or the
/// operation name, other than that the operation name is a UTF-8 encoded string.
#[must_use]
#[allow(missing_debug_implementations)]
pub struct WasmRsHost {
    engine: RefCell<Box<dyn WebAssemblyEngineProvider>>,
    id: u64,
}

impl WasmRsHost {
    pub fn next_id() -> u64 {
        GLOBAL_MODULE_COUNT.fetch_add(1, Ordering::SeqCst)
    }

    pub fn new(
        engine: Box<dyn WebAssemblyEngineProvider>,
        sync_hostcall: Option<Arc<HostCallback>>,
        async_hostcall: Option<Arc<AsyncHostCallback>>,
    ) -> Result<Self> {
        let id = Self::next_id();
        let mh = WasmRsHost {
            engine: RefCell::new(engine),
            id,
        };

        mh.initialize()?;

        Ok(mh)
    }

    fn initialize(&self) -> Result<()> {
        match self.engine.borrow_mut().init() {
            Ok(_) => Ok(()),
            Err(e) => Err(errors::Error::InitFailed(e.to_string())),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn new_context(&self) -> Result<WasmRsCallContext> {
        let state = Arc::new(ModuleState::new(self.id));
        let context = self
            .engine
            .borrow()
            .new_context(state.clone())
            .map_err(|e| crate::errors::Error::Context(e.to_string()))?;
        WasmRsCallContext::new(context, state)
    }
}

/// A builder for [WasmRsHost]s
#[must_use]
#[derive(Default)]
pub struct WasmRsHostBuilder {
    async_hostcall: Option<Arc<AsyncHostCallback>>,
    sync_hostcall: Option<Arc<HostCallback>>,
}

impl std::fmt::Debug for WasmRsHostBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmRsHostBuilder")
            .field(
                "async_hostcall",
                &self.async_hostcall.as_ref().map(|_| "Fn"),
            )
            .field("sync_hostcall", &self.sync_hostcall.as_ref().map(|_| "Fn"))
            .finish()
    }
}

impl WasmRsHostBuilder {
    /// Instantiate a new [WasmRsHostBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure a synchronous callback for this WasmRsHost.
    pub fn callback(mut self, callback: Arc<HostCallback>) -> Self {
        self.sync_hostcall = Some(callback);
        self
    }

    /// Configure a synchronous callback for this WasmRsHost.
    pub fn async_callback(mut self, callback: Arc<AsyncHostCallback>) -> Self {
        self.async_hostcall = Some(callback);
        self
    }

    /// Configure a synchronous callback for this WasmRsHost.
    pub fn build<T: WebAssemblyEngineProvider + 'static>(self, engine: T) -> Result<WasmRsHost> {
        WasmRsHost::new(Box::new(engine), self.sync_hostcall, self.async_hostcall)
    }
}

/// An isolated call context that is meant to be cheap to create and throw away.
pub struct WasmRsCallContext {
    context: Box<dyn ProviderCallContext + Send + Sync>,
    state: Arc<ModuleState>,
    id: u64,
}

impl std::fmt::Debug for WasmRsCallContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmRsCallContext")
            .field("state", &self.state)
            .finish()
    }
}

impl WasmRsCallContext {
    fn new(
        mut context: Box<dyn ProviderCallContext + Send + Sync>,
        state: Arc<ModuleState>,
    ) -> Result<Self> {
        let id = GLOBAL_CONTEXT_COUNT.fetch_add(1, Ordering::SeqCst);
        context
            .init()
            .map_err(|e| crate::errors::Error::InitFailed(e.to_string()))?;
        Ok(Self { context, state, id })
    }

    /// Return the unique id associated with this context.
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Invokes the `__guest_call` function within the guest module as per the wasmRS specification.
    /// Provide an operation name and an opaque payload of bytes and the function returns a `Result`
    /// containing either an error or an opaque reply of bytes.
    ///
    /// It is worth noting that the _first_ time `call` is invoked, the WebAssembly module
    /// might incur a "cold start" penalty, depending on which underlying engine you're using. This
    /// might be due to lazy initialization or JIT-compilation.
    pub fn request_response(
        &mut self,
        namespace: impl AsRef<str>,
        operation: impl AsRef<str>,
        data: Vec<u8>,
    ) -> Result<ShareableStream> {
        let (stream_id, stream) = self.state.new_stream();
        let metadata = Metadata::new(namespace, operation).encode();
        let payload = RequestPayload {
            stream_id,
            metadata,
            data,
            follows: false,
            complete: true,
            frame_type: wasmrs_rsocket::FrameType::RequestResponse,
            initial_n: 0,
        };
        let bytes = payload.encode();
        // let out = empty();

        match self.context.request_response(stream_id, bytes) {
            Ok(c) => c,
            Err(e) => {
                return Err(errors::Error::SendFailure(e.to_string()));
            }
        }

        Ok(stream)
    }
}
