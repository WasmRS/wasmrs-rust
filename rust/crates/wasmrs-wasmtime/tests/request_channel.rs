use std::collections::VecDeque;

use futures::StreamExt;
use wasmrs::{Metadata, Payload, RSocket};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
use wasmrs_wasmtime::WasmtimeBuilder;

static MODULE_BYTES: &[u8] = include_bytes!("../../../build/reqres_component.wasm");

#[test_log::test(tokio::test)]
async fn test_iota_wasm() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;
  let context = host.new_context()?;
  let op = context.get_export("suite.test", "reverse")?;

  let mbytes = Metadata::new(op).encode();

  let input = "HELLO WORLD".to_owned();

  let bytes = serialize(&input).unwrap();

  let payload = Payload::new(mbytes, bytes.into());

  let stream = Flux::new();
  stream.send(payload.clone())?;
  stream.complete();
  let mut response = context.request_channel(stream.take_rx().unwrap());
  let mut outputs: VecDeque<String> = vec!["DLROW OLLEH".to_owned()].into();
  while let Some(response) = response.next().await {
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
