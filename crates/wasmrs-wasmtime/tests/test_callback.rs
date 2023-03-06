use std::collections::VecDeque;

use futures::StreamExt;
use wasmrs::{GenericError, Metadata, Payload, PayloadError, RSocket, RawPayload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
use wasmrs_wasmtime::WasmtimeBuilder;

static MODULE_BYTES: &[u8] = include_bytes!("../../../build/baseline.wasm");

fn callback(
  incoming: FluxReceiver<Payload, PayloadError>,
) -> Result<FluxReceiver<RawPayload, PayloadError>, GenericError> {
  let (tx, rx) = FluxChannel::new_parts();
  tokio::spawn(async move {
    let mut incoming = incoming;
    while let Some(payload) = incoming.next().await {
      let _ = tx.send_result(payload.map(|p| RawPayload::new_data(None, Some(p.data))));
    }
  });
  Ok(rx)
}

#[test_log::test(tokio::test)]
async fn test_iota_req_channel_callback() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;

  host.register_request_channel("test", "callback", callback);
  let context = host.new_context()?;
  let op = context.get_export("test", "callback")?;

  let mbytes = Metadata::new(op).encode();

  let input = "HELLO WORLD".to_owned();

  let bytes = serialize(&input).unwrap();

  let payload = RawPayload::new(mbytes, bytes.into());

  let stream = FluxChannel::new();
  stream.send(payload.clone())?;
  stream.complete();
  let mut response = context.request_channel(Box::new(stream));
  let mut outputs: VecDeque<String> = vec!["HELLO WORLD".to_owned()].into();
  while let Some(response) = response.next().await {
    println!("response: {:?}", response);
    match response {
      Ok(v) => {
        let bytes = v.data.unwrap();
        let val: String = deserialize(&bytes).unwrap();
        println!("{}", val);
        let next = outputs.pop_front().unwrap();
        assert_eq!(val, next);
      }
      Err(e) => {
        panic!("Error: {:?}", e);
      }
    }
  }
  assert!(outputs.is_empty());

  Ok(())
}
