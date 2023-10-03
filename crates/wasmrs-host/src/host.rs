use std::sync::Arc;

use futures_core::Stream;
use futures_util::{FutureExt, StreamExt};
use parking_lot::Mutex;
use wasmrs::{
  BoxFlux, BoxMono, Frame, Handlers, IncomingMono, IncomingStream, Metadata, OperationHandler, OutgoingMono,
  OutgoingStream, Payload, RSocket, RawPayload, WasmSocket,
};
use wasmrs_frames::PayloadError;
use wasmrs_runtime::{spawn, ConditionallySend, UnboundedReceiver};
use wasmrs_rx::*;

use crate::context::{EngineProvider, SharedContext};

type Result<T> = std::result::Result<T, crate::errors::Error>;

#[must_use]
#[allow(missing_debug_implementations)]
/// A wasmRS native Host.
pub struct Host {
  engine: Box<dyn EngineProvider + Send + Sync>,
  mtu: usize,
  handlers: Arc<Mutex<Handlers>>,
}

impl Host {
  /// Create a new [Host] with an [EngineProvider] implementation.
  pub async fn new<E: EngineProvider + Send + Sync + 'static>(engine: E) -> Result<Self> {
    let host = Host {
      engine: Box::new(engine),
      mtu: 256,
      handlers: Default::default(),
    };

    Ok(host)
  }

  /// Create a new [CallContext], a way to bucket calls together with the same memory and configuration.
  pub async fn new_context(&self, host_buffer_size: u32, guest_buffer_size: u32) -> Result<CallContext> {
    let mut socket = WasmSocket::new(
      HostServer {
        handlers: self.handlers.clone(),
      },
      wasmrs::SocketSide::Host,
    );
    let rx = socket.take_rx().unwrap();
    let socket = Arc::new(socket);

    let context = self.engine.new_context(socket.clone()).await?;

    context.init(host_buffer_size, guest_buffer_size).await?;

    CallContext::new(self.mtu, socket, context, rx)
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_response(
    &self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: OperationHandler<IncomingMono, OutgoingMono>,
  ) -> usize {
    self.handlers.lock().register_request_response(ns, op, handler)
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_stream(
    &self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: OperationHandler<IncomingMono, OutgoingStream>,
  ) -> usize {
    self.handlers.lock().register_request_stream(ns, op, handler)
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_request_channel(
    &self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: OperationHandler<IncomingStream, OutgoingStream>,
  ) -> usize {
    self.handlers.lock().register_request_channel(ns, op, handler)
  }

  /// Register a Request/Response style handler on the host.
  pub fn register_fire_and_forget(
    &self,
    ns: impl AsRef<str>,
    op: impl AsRef<str>,
    handler: OperationHandler<IncomingMono, ()>,
  ) -> usize {
    self.handlers.lock().register_fire_and_forget(ns, op, handler)
  }
}

fn spawn_writer(mut rx: UnboundedReceiver<Frame>, context: SharedContext) -> tokio::task::JoinHandle<()> {
  spawn("host:spawn_writer", async move {
    while let Some(frame) = rx.recv().await {
      let _ = context.write_frame(frame).await;
    }
  })
}

#[allow(missing_debug_implementations)]
#[derive(Clone)]
/// A wasmRS native Host.
pub struct HostServer {
  handlers: Arc<Mutex<Handlers>>,
}

fn parse_payload(req: RawPayload) -> Payload {
  if let Some(mut md_bytes) = req.metadata {
    let md = Metadata::decode(&mut md_bytes).unwrap();
    Payload::new(md, req.data.unwrap())
  } else {
    panic!("No metadata found in payload.");
  }
}

impl RSocket for HostServer {
  fn fire_and_forget(&self, req: RawPayload) -> BoxMono<(), PayloadError> {
    let payload = parse_payload(req);
    let handler = self
      .handlers
      .lock()
      .get_fnf_handler(payload.metadata.index.unwrap())
      .unwrap();
    handler(futures_util::future::ready(Ok(payload)).boxed()).unwrap();
    futures_util::future::ready(Ok(())).boxed()
  }

  fn request_response(&self, req: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    let payload = parse_payload(req);
    let handler = self
      .handlers
      .lock()
      .get_request_response_handler(payload.metadata.index.unwrap())
      .unwrap();

    handler(futures_util::future::ready(Ok(payload)).boxed()).unwrap()
  }

  fn request_stream(&self, req: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    let payload = parse_payload(req);
    let handler = self
      .handlers
      .lock()
      .get_request_stream_handler(payload.metadata.index.unwrap())
      .unwrap();
    handler(futures_util::future::ready(Ok(payload)).boxed()).unwrap()
  }

  fn request_channel<
    T: Stream<Item = std::result::Result<RawPayload, PayloadError>> + ConditionallySend + Unpin + 'static,
  >(
    &self,
    mut reqs: T,
  ) -> BoxFlux<RawPayload, PayloadError> {
    let (out_tx, out_rx) = FluxChannel::<RawPayload, PayloadError>::new_parts();
    let handlers = self.handlers.clone();
    tokio::spawn(async move {
      let (inner_tx, inner_rx) = FluxChannel::new_parts();
      let first = match reqs.next().await {
        None => {
          let _ = out_tx.send_result(Err(PayloadError::application_error("No first payload.", None)));
          return;
        }
        Some(Err(e)) => {
          let _ = out_tx.send_result(Err(e));
          return;
        }
        Some(Ok(p)) => p,
      };

      let payload = parse_payload(first);
      let handler = handlers
        .lock()
        .get_request_channel_handler(payload.metadata.index.unwrap())
        .unwrap();
      let _ = inner_tx.send(payload);
      let mut out = handler(inner_rx.boxed()).unwrap();
      tokio::spawn(async move {
        while let Some(p) = out.next().await {
          let _ = out_tx.send_result(p);
        }
        out_tx.complete();
      });
      tokio::spawn(async move {
        while let Some(p) = reqs.next().await {
          let _ = inner_tx.send_result(p.map(parse_payload));
        }
        inner_tx.complete();
      });
    });
    out_rx.boxed()
  }
}

/// A [CallContext] is a way to bucket calls together with the same memory and configuration.
pub struct CallContext {
  socket: Arc<WasmSocket<HostServer>>,
  context: SharedContext,
  writer: tokio::task::JoinHandle<()>,
}

impl std::fmt::Debug for CallContext {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("WasmRsCallContext")
      .field("state", &self.socket)
      .finish()
  }
}

impl CallContext {
  fn new(
    _mtu: usize,
    socket: Arc<WasmSocket<HostServer>>,
    context: SharedContext,
    rx: UnboundedReceiver<Frame>,
  ) -> Result<Self> {
    let writer = spawn_writer(rx, context.clone());

    Ok(Self {
      socket,
      context,
      writer,
    })
  }

  /// Get the import id for a given namespace and operation.
  pub fn get_import(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.context.get_import(namespace, operation)
  }

  /// Get the export id for a given namespace and operation.
  pub fn get_export(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.context.get_export(namespace, operation)
  }

  /// Get a list of the exports for this context.
  #[must_use]
  pub fn get_exports(&self) -> Vec<String> {
    self.context.get_operation_list().get_exports()
  }

  /// A utility function to dump the operation list.
  pub fn dump_operations(&self) {
    println!("{:#?}", self.context.get_operation_list());
  }

  /// Query if the frame writer is still running.
  pub fn is_alive(&self) -> bool {
    !self.writer.is_finished()
  }
}

impl RSocket for CallContext {
  fn fire_and_forget(&self, payload: RawPayload) -> BoxMono<(), PayloadError> {
    self.socket.fire_and_forget(payload)
  }

  fn request_response(&self, payload: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    self.socket.request_response(payload)
  }

  fn request_stream(&self, payload: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    self.socket.request_stream(payload)
  }

  fn request_channel<
    T: Stream<Item = std::result::Result<RawPayload, PayloadError>> + ConditionallySend + Unpin + 'static,
  >(
    &self,
    stream: T,
  ) -> BoxFlux<RawPayload, PayloadError> {
    self.socket.request_channel(stream)
  }
}
