pub(crate) mod modulestate;
pub(crate) mod traits;

use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use futures_core::Stream;
use parking_lot::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::yield_now;
use wasmrs_rsocket::fragmentation::Splitter;
use wasmrs_rsocket::{runtime, ErrorCode, Frame, Metadata, Payload, PayloadError};
use wasmrs_rsocket::{PayloadFrame, RequestPayload};

use self::modulestate::ModuleState;
use self::traits::{SharedContext, WebAssemblyEngineProvider};
use crate::{errors, AsyncHostCallback, Handler, HostCallback, Invocation, ProviderCallContext};

static GLOBAL_MODULE_COUNT: AtomicU64 = AtomicU64::new(1);
static GLOBAL_CONTEXT_COUNT: AtomicU64 = AtomicU64::new(1);

type Result<T> = std::result::Result<T, crate::errors::Error>;

type ArcMutContext = Arc<Mutex<dyn ProviderCallContext + Send + Sync>>;

#[must_use]
#[allow(missing_debug_implementations)]
pub struct WasmRsHost {
    engine: RefCell<Box<dyn WebAssemblyEngineProvider>>,
    id: u64,
    mtu: usize,
}

impl WasmRsHost {
    pub fn next_id() -> u64 {
        GLOBAL_MODULE_COUNT.fetch_add(1, Ordering::SeqCst)
    }

    pub fn new(engine: Box<dyn WebAssemblyEngineProvider>) -> Result<Self> {
        let id = Self::next_id();
        let mh = WasmRsHost {
            engine: RefCell::new(engine),
            mtu: 256,
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
        WasmRsCallContext::new(self.mtu, context, state)
    }
}

/// A builder for [WasmRsHost]s
#[must_use]
#[derive(Default, Copy, Clone)]
pub struct WasmRsHostBuilder {}

impl std::fmt::Debug for WasmRsHostBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmRsHostBuilder").finish()
    }
}

impl WasmRsHostBuilder {
    /// Instantiate a new [WasmRsHostBuilder].
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure a synchronous callback for this WasmRsHost.
    pub fn build<T: WebAssemblyEngineProvider + 'static>(self, engine: T) -> Result<WasmRsHost> {
        WasmRsHost::new(Box::new(engine))
    }
}

/// An isolated call context that is meant to be cheap to create and throw away.
pub struct WasmRsCallContext {
    context: SharedContext,
    state: Arc<ModuleState>,
    splitter: Option<Splitter>,
    sender: UnboundedSender<Frame>,
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
    fn new(mtu: usize, mut context: SharedContext, state: Arc<ModuleState>) -> Result<Self> {
        let id = GLOBAL_CONTEXT_COUNT.fetch_add(1, Ordering::SeqCst);
        context
            .init()
            .map_err(|e| crate::errors::Error::InitFailed(e.to_string()))?;

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Frame>();
        let inner_state = state.clone();
        runtime::spawn(async move {
            while let Some(frame) = receiver.recv().await {
                let stream_id = frame.stream_id();
                trace!("Received frame from stream {}", stream_id);
                if let Frame::ErrorFrame(e) = &frame {
                    if e.stream_id == 0 {
                        error!("wasmrs internal error (code:{}, data:{})", e.code, e.data);
                        break;
                    }
                }
                if let Err(e) = inner_state.kick_handler(stream_id, frame.into()) {
                    error!("write frame failed: {}", e);
                    break;
                };
            }
        });

        let splitter = if mtu == 0 {
            None
        } else {
            Some(Splitter::new(mtu))
        };
        Ok(Self {
            context,
            sender,
            state,
            id,
            splitter,
        })
    }

    pub async fn request_response(&mut self, payload: Payload) -> Result<Option<Payload>> {
        let (tx, rx) =
            tokio::sync::oneshot::channel::<std::result::Result<Option<Payload>, PayloadError>>();
        let handler = Handler::ReqRR(tx);
        let stream_id = self.state.new_stream(handler);

        let start = std::time::Instant::now();
        send_request_response_payload(self.context.clone(), self.splitter, stream_id, payload);
        let end = std::time::Instant::now();
        println!("write frame duration: {}ns", (end - start).as_nanos());
        match rx.await {
            Ok(v) => {
                let end = std::time::Instant::now();
                println!("total request duration: {}ns", (end - start).as_nanos());
                Ok(v?)
            }
            Err(e) => Err(wasmrs_rsocket::Error::RequestResponse(e.to_string()).into()),
        }
    }
}

fn send_request_response_payload(
    context: SharedContext,
    splitter: Option<Splitter>,
    stream_id: u32,
    payload: Payload,
) {
    match splitter {
        None => {
            let sending = Frame::new_request_response(stream_id, payload, Frame::FLAG_FOLLOW, 0);
            let _ = context.write_frame(stream_id, sending);
        }
        Some(sp) => {
            let mut cuts: usize = 0;
            let mut prev: Option<Payload> = None;
            for next in sp.cut(payload, 0) {
                if let Some(cur) = prev.take() {
                    let sending = if cuts == 1 {
                        // make first frame as request_response.
                        Frame::new_request_response(stream_id, cur, Frame::FLAG_FOLLOW, 0)
                    } else {
                        // make other frames as payload.
                        Frame::new_payload(stream_id, cur, Frame::FLAG_FOLLOW)
                    };
                    if context.write_frame(stream_id, sending).is_err() {
                        return;
                    }
                }
                prev = Some(next);
                cuts += 1;
            }

            let sending = if cuts == 0 {
                Frame::new_request_response(stream_id, Payload::empty(), 0, 0)
            } else if cuts == 1 {
                Frame::new_request_response(stream_id, prev.unwrap_or_default(), 0, 0)
            } else {
                Frame::new_payload(stream_id, prev.unwrap_or_default(), 0)
            };
            let _ = context.write_frame(stream_id, sending);
        }
    }
}
