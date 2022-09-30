use std::cell::RefCell;
use std::sync::Arc;

use wasmrs::flux::FluxReceiver;
use wasmrs::runtime::{spawn, UnboundedReceiver};
use wasmrs::{Frame, Payload, PayloadError, RSocket, WasmSocket};

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
        let mut socket = WasmSocket::new(HostServer {}, 1);
        let rx = socket.take_rx().unwrap();
        let state = Arc::new(socket);

        let context = self.engine.borrow().new_context(state.clone())?;
        context.init()?;
        spawn_writer(rx, context);

        CallContext::new(self.mtu, state)
    }
}

fn spawn_writer(mut rx: UnboundedReceiver<Frame>, context: SharedContext) {
    spawn(async move {
        while let Some(frame) = rx.recv().await {
            let _ = context.write_frame(frame.stream_id(), frame);
        }
    });
}

struct HostServer {}

impl RSocket for HostServer {
    fn fire_and_forget(&self, _req: Payload) -> FluxReceiver<(), PayloadError> {
        todo!()
    }

    fn request_response(&self, _payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        todo!();
    }

    fn request_stream(&self, _req: Payload) -> FluxReceiver<Payload, PayloadError> {
        todo!()
    }

    fn request_channel(
        &self,
        _reqs: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        todo!()
    }
}

pub struct CallContext {
    socket: Arc<WasmSocket>,
}

impl std::fmt::Debug for CallContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmRsCallContext")
            .field("state", &self.socket)
            .finish()
    }
}

impl CallContext {
    fn new(_mtu: usize, state: Arc<WasmSocket>) -> Result<Self> {
        Ok(Self { socket: state })
    }
}

impl RSocket for CallContext {
    fn fire_and_forget(&self, _payload: Payload) -> FluxReceiver<(), PayloadError> {
        todo!()
    }

    fn request_response(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        self.socket.request_response(payload)
    }

    fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        self.socket.request_stream(payload)
    }

    fn request_channel(
        &self,
        stream: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        self.socket.request_channel(stream)
    }
}
