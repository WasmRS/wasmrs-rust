use std::{io::Read, sync::Arc};

use base64::Engine;
use clap::Parser;
use futures::{FutureExt, StreamExt};
use tracing::{debug, info};
use wasmrs::{BoxFlux, BoxMono, RSocket, RawPayload, SocketSide, WasmSocket};
use wasmrs_frames::PayloadError;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
use wasmrs_testhost::WasmtimeBuilder;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
  /// Wasm module
  #[arg()]
  module: String,

  /// Replay file
  #[arg()]
  replay: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::Builder::new().parse_filters("wasmrs=info").init();
  let args = Args::parse();
  info!("running replay {} against {}", args.replay, args.module);

  let module_bytes = std::fs::read(args.module)?;
  let engine = WasmtimeBuilder::new(&module_bytes)
    .wasi_params(WasiParams::default())
    .build()?;
  let socket_impl = HostServer {};

  let mut socket = WasmSocket::new(socket_impl, SocketSide::Host);

  let mut rx = socket.take_rx().unwrap();
  let mut context = engine.new_context(Arc::new(socket))?;
  context.run_init(64 * 1024, 64 * 1024)?;

  let mut replay = String::new();
  std::fs::File::open(args.replay)?.read_to_string(&mut replay)?;

  let mut handled = 0;
  for line in replay.lines() {
    let record: wasmrs::FrameRecord = serde_json::from_str(line)?;
    if record.is_outgoing() {
      debug!("sending frame: {}", record);

      let decoded = record.frame()?;

      let result = context.send_frame(decoded);
      assert!(result.is_ok());
      handled += 1;
    } else {
      debug!("waiting for frame: {}", record);

      if let Some(frame) = rx.recv().await {
        let bytes = frame.encode();
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        debug!("got frame: {}", encoded);
        assert_eq!(encoded.as_str(), record.encoded());
        handled += 1;
      } else {
        panic!("No frame received");
      }
    }
  }
  assert_eq!(handled, replay.lines().count());

  info!("done!");

  Ok(())
}

struct HostServer {}

impl RSocket for HostServer {
  fn fire_and_forget(&self, _req: RawPayload) -> BoxMono<(), PayloadError> {
    Mono::default().boxed()
  }

  fn request_response(&self, _payload: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    Mono::default().boxed()
  }

  fn request_stream(&self, _req: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    let (tx, rx) = FluxChannel::new_parts();
    tx.complete();
    rx.boxed()
  }

  fn request_channel(&self, _reqs: BoxFlux<RawPayload, PayloadError>) -> BoxFlux<RawPayload, PayloadError> {
    let (tx, rx) = FluxChannel::new_parts();
    tx.complete();
    rx.boxed()
  }
}
