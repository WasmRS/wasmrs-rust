use std::collections::VecDeque;

use futures::StreamExt;
use wasmrs::{BoxFlux, GenericError, Metadata, Payload, PayloadError, RSocket, RawPayload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
use wasmrs_wasmtime::WasmtimeBuilder;

static MODULE_BYTES: &[u8] = include_bytes!("../../../build/baseline.wasm");

fn callback(incoming: BoxFlux<Payload, PayloadError>) -> Result<BoxFlux<RawPayload, PayloadError>, GenericError> {
  let (tx, rx) = FluxChannel::new_parts();
  tokio::spawn(async move {
    let mut incoming = incoming;
    while let Some(payload) = incoming.next().await {
      let _ = tx.send_result(payload.map(|p| RawPayload::new_data(None, Some(p.data))));
    }
  });
  Ok(rx.boxed())
}

#[test_log::test(tokio::test)]
async fn test_req_channel_callback() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;

  host.register_request_channel("test", "callback", Box::new(callback));
  let context = host.new_context(64 * 1024, 64 * 1024)?;
  let op = context.get_export("test", "callback")?;

  let mbytes = Metadata::new(op).encode();

  let input = "HELLO WORLD".to_owned();

  let bytes = serialize(&input).unwrap();

  let payload = RawPayload::new(mbytes, bytes.into());

  let stream = FluxChannel::new();
  stream.send(payload.clone())?;
  stream.complete();
  let mut response = context.request_channel(Box::pin(stream));
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

#[test_log::test(tokio::test)]
async fn test_req_res() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;

  let buffer_size = 10 * 1024 * 1024;
  let context = host.new_context(buffer_size, buffer_size)?;
  let op = context.get_export("greeting", "sayHello")?;

  let mbytes = Metadata::new(op).encode();

  #[derive(serde::Serialize)]
  struct Input {
    message: Vec<String>,
  }
  let message = "01234567";

  let mut input = Input { message: vec![] };
  let mb = 1024 * 1024;

  for _ in 0..mb {
    input.message.push(message.to_string());
  }

  let bytes = serialize(&input).unwrap();

  let payload = RawPayload::new(mbytes, bytes.into());

  println!("making large request");
  let num = input.message.len();
  let response = context.request_response(payload.clone()).await;
  println!("finished large request");
  match response {
    Ok(v) => {
      let bytes = v.data.unwrap();
      let val: String = deserialize(&bytes).unwrap();
      println!("{}", val);
      assert_eq!(val, format!("Hello! You sent me {} messages.", num));
    }
    Err(e) => {
      panic!("Error: {:?}", e);
    }
  }

  Ok(())
}
