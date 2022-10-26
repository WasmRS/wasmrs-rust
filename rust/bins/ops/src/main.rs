use clap::Parser;
use futures::StreamExt;
use wasmrs::{Metadata, Payload, RSocket};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeBuilder;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
  /// Wasm module
  #[arg()]
  module: String,
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
  context.dump_operations();

  Ok(())
}
