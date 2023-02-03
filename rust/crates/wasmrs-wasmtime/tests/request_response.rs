use wasmrs::{Metadata, Payload, RSocket};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeBuilder;

static MODULE_BYTES: &[u8] = include_bytes!("../../../build/reqres_component.wasm");

#[test_log::test(tokio::test)]
async fn test_iota_req_response() -> anyhow::Result<()> {
  let engine = WasmtimeBuilder::new(MODULE_BYTES)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;
  let context = host.new_context()?;
  let op = context.get_export("suite.test", "echo")?;

  let mbytes = Metadata::new(op).encode();

  #[derive(serde::Serialize)]
  struct Input {
    message: String,
  }
  let input = Input {
    message: "HELLO WORLD".to_owned(),
  };

  let bytes = serialize(&input).unwrap();

  let payload = Payload::new(mbytes, bytes.into());

  let response = context.request_response(payload.clone());
  match response.await {
    Ok(v) => {
      let bytes = v.data.unwrap();
      let val: String = deserialize(&bytes).unwrap();
      println!("{}", val);
      assert_eq!(val, "HELLO WORLD");
    }
    Err(e) => {
      panic!("Error: {:?}", e);
    }
  }

  Ok(())
}
