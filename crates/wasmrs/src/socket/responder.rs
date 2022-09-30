use std::sync::Arc;

use parking_lot::RwLock;

use crate::{flux::*, ErrorCode, Payload, PayloadError, RSocket};

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
    fn fire_and_forget(&self, req: Payload) -> FluxReceiver<(), PayloadError> {
        let inner = self.inner.read();
        (*inner).fire_and_forget(req)
    }

    fn request_response(&self, req: Payload) -> FluxReceiver<Payload, PayloadError> {
        let inner = self.inner.read();
        (*inner).request_response(req)
    }

    fn request_stream(&self, req: Payload) -> FluxReceiver<Payload, PayloadError> {
        let inner = self.inner.clone();
        let r = inner.read();
        (*r).request_stream(req)
    }

    fn request_channel(
        &self,
        stream: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        let inner = self.inner.clone();
        let r = inner.read();
        (*r).request_channel(stream)
    }
}
pub(crate) struct EmptyRSocket;

impl RSocket for EmptyRSocket {
    fn fire_and_forget(&self, _req: Payload) -> FluxReceiver<(), PayloadError> {
        let channel = Flux::<(), PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.split_receiver().unwrap()
    }

    fn request_response(&self, _req: Payload) -> FluxReceiver<Payload, PayloadError> {
        let channel = Flux::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.split_receiver().unwrap()
    }

    fn request_stream(&self, _req: Payload) -> FluxReceiver<Payload, PayloadError> {
        let channel = Flux::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.split_receiver().unwrap()
    }

    fn request_channel(
        &self,
        _reqs: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        let channel = Flux::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::new(
            ErrorCode::ApplicationError.into(),
            "Unimplemented",
        ));
        channel.split_receiver().unwrap()
    }
}
