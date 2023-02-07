use std::sync::Arc;

use base64::Engine;
use wasmrs::{Payload, RSocket, SocketSide, WasmSocket};
use wasmrs_frames::PayloadError;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
use wasmrs_testhost::WasmtimeBuilder;

static MODULE_BYTES: &[u8] = include_bytes!("../../../build/reqres_component.wasm");
static REPLAYS: [&str; 2] = [
  include_str!("../replay-reqres.replay"),
  include_str!("../replay-reqchannel.replay"),
];

#[test_log::test(tokio::test)]
async fn test_iota_req_channel() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let socket_impl = HostServer {};

  let mut socket = WasmSocket::new(socket_impl, SocketSide::Host);

  let mut rx = socket.take_rx().unwrap();
  let mut context = engine.new_context(Arc::new(socket))?;
  context.run_init()?;

  for replay in REPLAYS {
    let mut handled = 0;
    for line in replay.lines() {
      let record: wasmrs::FrameRecord = serde_json::from_str(line)?;
      if record.is_outgoing() {
        println!("sending frame: {}", record);

        let decoded = record.frame()?;

        let result = context.send_frame(decoded);
        assert!(result.is_ok());
        handled += 1;
      } else {
        println!("waiting for frame: {}", record);
        if let Some(frame) = rx.recv().await {
          let bytes = frame.encode();
          let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
          println!("got frame: {}", encoded);
          assert_eq!(encoded.as_str(), record.encoded());
          handled += 1;
        } else {
          panic!("No frame received");
        }
      }
    }
    assert_eq!(handled, replay.lines().count());
  }

  println!("done!");

  Ok(())
}

struct HostServer {}

impl RSocket for HostServer {
  fn fire_and_forget(&self, _req: Payload) -> Mono<(), PayloadError> {
    Mono::default()
  }

  fn request_response(&self, _payload: Payload) -> Mono<Payload, PayloadError> {
    Mono::default()
  }

  fn request_stream(&self, _req: Payload) -> FluxReceiver<Payload, PayloadError> {
    let (tx, rx) = Flux::new_channels();
    tx.complete();
    rx
  }

  fn request_channel(&self, _reqs: FluxReceiver<Payload, PayloadError>) -> FluxReceiver<Payload, PayloadError> {
    let (tx, rx) = Flux::new_channels();
    tx.complete();
    rx
  }
}
