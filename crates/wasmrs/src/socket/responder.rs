use std::sync::Arc;

use parking_lot::RwLock;

use crate::{flux::*, ErrorCode, Mono, Payload, PayloadError, RSocket};

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
    fn fire_and_forget(&self, req: Payload) -> Mono<(), PayloadError> {
        let inner = self.inner.read();
        (*inner).fire_and_forget(req)
    }

    fn request_response(&self, req: Payload) -> Mono<Payload, PayloadError> {
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
    fn fire_and_forget(&self, _req: Payload) -> Mono<(), PayloadError> {
        Mono::new_error(PayloadError::application_error("Unimplemented"))
    }

    fn request_response(&self, _req: Payload) -> Mono<Payload, PayloadError> {
        Mono::new_error(PayloadError::application_error("Unimplemented"))
    }

    fn request_stream(&self, _req: Payload) -> FluxReceiver<Payload, PayloadError> {
        let channel = Flux::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::application_error("Unimplemented"));
        channel.split_receiver().unwrap()
    }

    fn request_channel(
        &self,
        _reqs: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        let channel = Flux::<Payload, PayloadError>::new();
        let _ = channel.error(PayloadError::application_error("Unimplemented"));
        channel.split_receiver().unwrap()
    }
}
