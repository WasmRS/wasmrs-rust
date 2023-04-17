use bytes::Bytes;
use wasmrs_frames::{Metadata, PayloadError, RawPayload};
use wasmrs_runtime::RtRc;

use crate::operations::OperationList;
use crate::{BoxFlux, BoxMono};

/// An alias to [Box<dyn std::error::Error + Send + Sync + 'static>]
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
/// An alias for a [Vec<(String, String, RtRc<T>)>]
pub type OperationMap<T> = Vec<(String, String, RtRc<T>)>;
/// An alias for the function that creates the output for a task.
pub type ProcessFactory<I, O> = Box<dyn Fn(I) -> Result<O, GenericError> + Send + Sync>;

/// An alias for [Mono<ParsedPayload, PayloadError>]
pub type IncomingMono = BoxMono<Payload, PayloadError>;
/// An alias for [Mono<Payload, PayloadError>]
pub type OutgoingMono = BoxMono<RawPayload, PayloadError>;
/// An alias for [FluxReceiver<ParsedPayload, PayloadError>]
pub type IncomingStream = BoxFlux<Payload, PayloadError>;
/// An alias for [FluxReceiver<Payload, PayloadError>]
pub type OutgoingStream = BoxFlux<RawPayload, PayloadError>;

#[allow(missing_debug_implementations)]
#[derive(Debug)]
/// A [Payload] with pre-parsed [Metadata].
pub struct Payload {
  /// The parsed [Metadata].
  pub metadata: Metadata,
  /// The raw data bytes.
  pub data: Bytes,
}

impl Payload {
  /// Create a new [ParsedPayload] from the given [Metadata] and [Bytes].
  pub fn new(metadata: Metadata, data: Bytes) -> Self {
    Self { metadata, data }
  }
}

impl TryFrom<RawPayload> for Payload {
  type Error = crate::Error;

  fn try_from(mut value: RawPayload) -> Result<Self, Self::Error> {
    Ok(Payload {
      metadata: value.parse_metadata()?,
      data: value.data.unwrap_or_default(),
    })
  }
}

#[derive(Default)]
/// A list of all the operations exported by a wasmrs implementer.
pub struct Handlers {
  op_list: OperationList,
  request_response_handlers: OperationMap<ProcessFactory<IncomingMono, OutgoingMono>>,
  request_stream_handlers: OperationMap<ProcessFactory<IncomingMono, OutgoingStream>>,
  request_channel_handlers: OperationMap<ProcessFactory<IncomingStream, OutgoingStream>>,
  request_fnf_handlers: OperationMap<ProcessFactory<IncomingMono, ()>>,
}

impl std::fmt::Debug for Handlers {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Handlers").field("op_list", &self.op_list).finish()
  }
}

impl Handlers {
  /// Get the operation list.
  pub fn op_list(&self) -> &OperationList {
    &self.op_list
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_response(
    &mut self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: ProcessFactory<IncomingMono, OutgoingMono>,
  ) -> usize {
    let list = &mut self.request_response_handlers;
    list.push((ns.as_ref().to_owned(), op.as_ref().to_owned(), RtRc::new(handler)));
    let index = list.len() - 1;
    self
      .op_list
      .add_export(index as _, crate::OperationType::RequestResponse, ns, op);
    index
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_stream(
    &mut self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: ProcessFactory<IncomingMono, OutgoingStream>,
  ) -> usize {
    let list = &mut self.request_stream_handlers;
    list.push((ns.as_ref().to_owned(), op.as_ref().to_owned(), RtRc::new(handler)));
    let index = list.len() - 1;
    self
      .op_list
      .add_export(index as _, crate::OperationType::RequestStream, ns, op);
    index
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_channel(
    &mut self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: ProcessFactory<IncomingStream, OutgoingStream>,
  ) -> usize {
    let list = &mut self.request_channel_handlers;
    list.push((ns.as_ref().to_owned(), op.as_ref().to_owned(), RtRc::new(handler)));
    let index = list.len() - 1;
    self
      .op_list
      .add_export(index as _, crate::OperationType::RequestChannel, ns, op);
    index
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_fire_and_forget(
    &mut self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: ProcessFactory<IncomingMono, ()>,
  ) -> usize {
    let list = &mut self.request_fnf_handlers;
    list.push((ns.as_ref().to_owned(), op.as_ref().to_owned(), RtRc::new(handler)));
    let index = list.len() - 1;
    self
      .op_list
      .add_export(index as _, crate::OperationType::RequestFnF, ns, op);
    index
  }

  #[must_use]
  /// Get a Request/Response handler by id.
  pub fn get_request_response_handler(&self, index: u32) -> Option<RtRc<ProcessFactory<IncomingMono, OutgoingMono>>> {
    let a = self
      .request_response_handlers
      .get(index as usize)
      .map(|(_, _, h)| h.clone());
    a
  }
  #[must_use]
  /// Get a Request/Response handler by id.
  pub fn get_request_stream_handler(&self, index: u32) -> Option<RtRc<ProcessFactory<IncomingMono, OutgoingStream>>> {
    let a = self
      .request_stream_handlers
      .get(index as usize)
      .map(|(_, _, h)| h.clone());
    a
  }
  #[must_use]
  /// Get a Request/Response handler by id.
  pub fn get_request_channel_handler(
    &self,
    index: u32,
  ) -> Option<RtRc<ProcessFactory<IncomingStream, OutgoingStream>>> {
    let a = self
      .request_channel_handlers
      .get(index as usize)
      .map(|(_, _, h)| h.clone());
    a
  }
  #[must_use]
  /// Get a Request/Response handler by id.
  pub fn get_fnf_handler(&self, index: u32) -> Option<RtRc<ProcessFactory<IncomingMono, ()>>> {
    let a = self.request_fnf_handlers.get(index as usize).map(|(_, _, h)| h.clone());
    a
  }
}
