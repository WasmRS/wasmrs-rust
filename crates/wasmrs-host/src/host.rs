use std::cell::RefCell;
use std::sync::Arc;

use wasmrs::flux::{FluxChannel, FluxStream};
use wasmrs::fragmentation::Splitter;
use wasmrs::runtime::{spawn, Receiver};
use wasmrs::{Frame, Handler, Payload, PayloadError, RSocket, SocketManager};

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
        let mut manager = SocketManager::new(HostServer {});
        let rx = manager.take_rx().unwrap();
        let state = Arc::new(manager);

        let context = self.engine.borrow().new_context(state.clone())?;
        spawn_writer(rx, context.clone());

        CallContext::new(self.mtu, context, state)
    }
}

fn spawn_writer(mut rx: Receiver<Frame>, context: SharedContext) {
    spawn(async move {
        while let Some(frame) = rx.recv().await {
            let _ = context.write_frame(frame.stream_id(), frame);
        }
    });
}

struct HostServer {}

impl RSocket for HostServer {
    fn fire_and_forget(&self, _req: Payload) -> FluxStream<(), PayloadError> {
        todo!()
    }

    fn request_response(&self, _payload: Payload) -> FluxStream<Payload, PayloadError> {
        todo!();
    }

    fn request_stream(&self, _req: Payload) -> FluxStream<Payload, PayloadError> {
        todo!()
    }

    fn request_channel(
        &self,
        _reqs: FluxChannel<Payload, PayloadError>,
    ) -> FluxStream<Payload, PayloadError> {
        todo!()
    }
}

/// An isolated call context that is meant to be cheap to create and throw away.
pub struct CallContext {
    context: SharedContext,
    state: Arc<SocketManager>,
    _splitter: Option<Splitter>,
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
            _splitter: splitter,
        })
    }

    pub async fn request_response(&mut self, payload: Payload) -> Result<Option<Payload>> {
        let (tx, rx) =
            tokio::sync::oneshot::channel::<std::result::Result<Option<Payload>, PayloadError>>();

        let handler = Handler::ReqRR(tx);
        let stream_id = self.state.new_stream(handler);

        let sending = Frame::new_request_response(stream_id, payload, Frame::FLAG_FOLLOW, 0);
        let _ = self.context.write_frame(stream_id, sending);

        match rx.await {
            Ok(v) => Ok(v?),
            Err(e) => Err(wasmrs::Error::RequestResponse(e.to_string()).into()),
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn request_stream(
        &mut self,
        input: Payload,
    ) -> Result<FluxStream<Payload, PayloadError>> {
        let flux = FluxChannel::new();
        let output = flux.observer().unwrap();

        let handler = Handler::ReqRC(flux);
        let stream_id = self.state.new_stream(handler);

        let sending = Frame::new_request_stream(stream_id, input, 0, 0);
        let _ = self.context.write_frame(stream_id, sending);
        Ok(output)
    }

    #[allow(clippy::unused_async)]
    pub async fn request_channel(
        &mut self,
        _payload: FluxChannel<Payload, Box<dyn std::error::Error + Send + Sync>>,
    ) -> Result<Option<Payload>> {
        todo!()
    }
}
