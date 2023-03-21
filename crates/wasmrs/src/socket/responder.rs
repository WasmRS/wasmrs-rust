use std::sync::Arc;

use parking_lot::RwLock;
use wasmrs_rx::*;

use crate::{Mono, PayloadError, RSocket, RawPayload};

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
  fn fire_and_forget(&self, req: RawPayload) -> Mono<(), PayloadError> {
    let inner = self.inner.read();
    (*inner).fire_and_forget(req)
  }

  fn request_response(&self, req: RawPayload) -> Mono<RawPayload, PayloadError> {
    let inner = self.inner.read();
    (*inner).request_response(req)
  }

  fn request_stream(&self, req: RawPayload) -> FluxReceiver<RawPayload, PayloadError> {
    let inner = self.inner.clone();
    let r = inner.read();
    (*r).request_stream(req)
  }

  fn request_channel(&self, stream: Box<dyn Flux<RawPayload, PayloadError>>) -> FluxReceiver<RawPayload, PayloadError> {
    let inner = self.inner.clone();
    let r = inner.read();
    (*r).request_channel(stream)
  }
}
pub(crate) struct EmptyRSocket;

impl RSocket for EmptyRSocket {
  fn fire_and_forget(&self, _req: RawPayload) -> Mono<(), PayloadError> {
    Mono::new_error(PayloadError::application_error("Unimplemented", None))
  }

  fn request_response(&self, _req: RawPayload) -> Mono<RawPayload, PayloadError> {
    Mono::new_error(PayloadError::application_error("Unimplemented", None))
  }

  fn request_stream(&self, _req: RawPayload) -> FluxReceiver<RawPayload, PayloadError> {
    let (tx, channel) = FluxChannel::<RawPayload, PayloadError>::new_parts();
    let _ = tx.error(PayloadError::application_error("Unimplemented", None));
    channel
  }

  fn request_channel(&self, _reqs: Box<dyn Flux<RawPayload, PayloadError>>) -> FluxReceiver<RawPayload, PayloadError> {
    let (tx, channel) = FluxChannel::<RawPayload, PayloadError>::new_parts();
    let _ = tx.error(PayloadError::application_error("Unimplemented", None));
    channel
  }
}