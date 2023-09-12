use std::sync::Arc;

use futures::{FutureExt, Stream, StreamExt};
use parking_lot::RwLock;
use wasmrs_runtime::ConditionallySend;

use crate::{BoxFlux, BoxMono, PayloadError, RSocket, RawPayload};

pub(crate) struct Responder<T> {
  inner: Arc<RwLock<T>>,
}

impl<T: RSocket> Clone for Responder<T> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
    }
  }
}

impl<T: RSocket> Responder<T> {
  pub(crate) fn new(rsocket: T) -> Responder<T> {
    Responder {
      inner: Arc::new(RwLock::new(rsocket)),
    }
  }
}

impl<T: RSocket> RSocket for Responder<T> {
  fn fire_and_forget(&self, req: RawPayload) -> BoxMono<(), PayloadError> {
    let inner = self.inner.read();
    (*inner).fire_and_forget(req)
  }

  fn request_response(&self, req: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    let inner = self.inner.read();
    (*inner).request_response(req)
  }

  fn request_stream(&self, req: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    let inner = self.inner.clone();
    let r = inner.read();
    (*r).request_stream(req)
  }

  fn request_channel<S: Stream<Item = Result<RawPayload, PayloadError>> + ConditionallySend + Unpin + 'static>(
    &self,
    stream: S,
  ) -> BoxFlux<RawPayload, PayloadError> {
    let inner = self.inner.clone();
    let r = inner.read();
    (*r).request_channel(stream)
  }
}
#[derive(Clone)]
pub(crate) struct EmptyRSocket;

impl RSocket for EmptyRSocket {
  fn fire_and_forget(&self, _req: RawPayload) -> BoxMono<(), PayloadError> {
    futures::future::ready(Err(PayloadError::application_error("Unimplemented", None))).boxed()
  }

  fn request_response(&self, _req: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    futures::future::ready(Err(PayloadError::application_error("Unimplemented", None))).boxed()
  }

  fn request_stream(&self, _req: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    futures::stream::iter([Err(PayloadError::application_error("Unimplemented", None))]).boxed()
  }

  fn request_channel<T: Stream<Item = Result<RawPayload, PayloadError>>>(
    &self,
    _reqs: T,
  ) -> BoxFlux<RawPayload, PayloadError> {
    futures::stream::iter([Err(PayloadError::application_error("Unimplemented", None))]).boxed()
  }
}
