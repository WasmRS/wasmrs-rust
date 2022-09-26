use std::cell::RefCell;
use std::sync::Arc;

use wasmrs::fragmentation::Splitter;
use wasmrs::{Frame, Handler, Payload, PayloadError, SocketManager};

use crate::context::{EngineProvider, SharedContext};

type Result<T> = std::result::Result<T, crate::errors::Error>;

#[must_use]
#[allow(missing_debug_implementations)]
pub struct Host {
    engine: RefCell<Box<dyn EngineProvider>>,
    mtu: usize,
}

impl Host {
    pub fn new<T: EngineProvider + 'static>(engine: T) -> Result<Self> {
        let host = Host {
            engine: RefCell::new(Box::new(engine)),
            mtu: 256,
        };

        host.engine.borrow_mut().init()?;

        Ok(host)
    }

    pub fn new_context(&self) -> Result<CallContext> {
        let state = Arc::new(SocketManager::new());

        let context = self.engine.borrow().new_context(state.clone())?;

        CallContext::new(self.mtu, context, state)
    }
}

/// An isolated call context that is meant to be cheap to create and throw away.
pub struct CallContext {
    context: SharedContext,
    state: Arc<SocketManager>,
    splitter: Option<Splitter>,
}

impl std::fmt::Debug for CallContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmRsCallContext")
            .field("state", &self.state)
            .finish()
    }
}

impl CallContext {
    fn new(mtu: usize, context: SharedContext, state: Arc<SocketManager>) -> Result<Self> {
        context.init()?;

        let splitter = if mtu == 0 {
            None
        } else {
            Some(Splitter::new(mtu))
        };
        Ok(Self {
            context,
            state,
            splitter,
        })
    }

    pub async fn request_response(&mut self, payload: Payload) -> Result<Option<Payload>> {
        let (tx, rx) =
            tokio::sync::oneshot::channel::<std::result::Result<Option<Payload>, PayloadError>>();
        let handler = Handler::ReqRR(tx);
        let stream_id = self.state.new_stream(handler);

        send_request_response_payload(self.context.clone(), self.splitter, stream_id, payload);

        match rx.await {
            Ok(v) => Ok(v?),
            Err(e) => Err(wasmrs::Error::RequestResponse(e.to_string()).into()),
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
