use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    flux::{FluxChannel, FluxStream},
    ErrorCode, Payload, PayloadError, RSocket,
};

#[derive(Clone)]
pub(crate) struct Responder {
    inner: Arc<RwLock<Box<dyn RSocket>>>,
}

impl Responder {
    pub(crate) fn new(rsocket: Box<dyn RSocket>) -> Responder {
        Responder {
            inner: Arc::new(RwLock::new(rsocket)),
        }
    }
}

impl RSocket for Responder {
    fn fire_and_forget(&self, req: Payload) -> FluxStream<(), PayloadError> {
        let inner = self.inner.read();
        (*inner).fire_and_forget(req)
    }

    fn request_response(&self, req: Payload) -> FluxStream<Payload, PayloadError> {
        let inner = self.inner.read();
        (*inner).request_response(req)
    }

    fn request_stream(&self, req: Payload) -> FluxStream<Payload, PayloadError> {
        let inner = self.inner.clone();
        let r = inner.read();
        (*r).request_stream(req)
    }

    fn request_channel(
        &self,
        stream: FluxChannel<Payload, PayloadError>,
    ) -> FluxStream<Payload, PayloadError> {
        let inner = self.inner.clone();
        let r = inner.read();
        (*r).request_channel(stream)
    }
}
pub(crate) struct EmptyRSocket;

impl RSocket for EmptyRSocket {
    fn fire_and_forget(&self, _req: Payload) -> FluxStream<(), PayloadError> {
        let channel = FluxChannel::<(), PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.observer().unwrap()
    }

    fn request_response(&self, _req: Payload) -> FluxStream<Payload, PayloadError> {
        let channel = FluxChannel::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.observer().unwrap()
    }

    fn request_stream(&self, _req: Payload) -> FluxStream<Payload, PayloadError> {
        let channel = FluxChannel::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.observer().unwrap()
    }

    fn request_channel(
        &self,
        _reqs: FluxChannel<Payload, PayloadError>,
    ) -> FluxStream<Payload, PayloadError> {
        let channel = FluxChannel::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.observer().unwrap()
    }
}
