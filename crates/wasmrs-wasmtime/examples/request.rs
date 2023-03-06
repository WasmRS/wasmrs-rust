use clap::Parser;
use wasmrs::{Metadata, RSocket, RawPayload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeBuilder;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
  /// Wasm module
  #[arg()]
  module: String,

  /// Namespace
  #[arg()]
  namespace: String,

  /// Operation
  #[arg()]
  operation: String,

  /// Data to send
  #[arg()]
  data: String,

  /// Treat request as request_stream
  #[arg(long = "stream", short = 's')]
  stream: bool,

  /// Treat request as request_channel
  #[arg(long = "channel", short = 'c')]
  channel: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  let args = Args::parse();

  let module_bytes = std::fs::read(args.module)?;
  let engine = WasmtimeBuilder::new(&module_bytes)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;
  let context = host.new_context()?;

  let op = context.get_export(&args.namespace, &args.operation)?;

  let mbytes = Metadata::new(op).encode();
  let val: serde_json::Value = serde_json::from_str(&args.data)?;
  let bytes = serialize(&val).unwrap();

  let payload = RawPayload::new(mbytes, bytes.into());

  if args.stream {
    unimplemented!()
  } else if args.channel {
    unimplemented!()
  } else {
    let response = context.request_response(payload.clone());
    match response.await {
      Ok(v) => {
        let bytes = v.data.unwrap();
        let val: String = deserialize(&bytes).unwrap();
        println!("{}", val);
      }
      Err(e) => {
        println!("Error: {}", e)
      }
    }
  }

  Ok(())
}
