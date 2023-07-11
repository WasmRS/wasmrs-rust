use clap::Parser;
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

  let module_bytes = std::fs::read(&args.module)?;
  let engine = WasmtimeBuilder::new()
    .with_module_bytes(&args.module, &module_bytes)
    .wasi_params(WasiParams::default())
    .build()?;
  let host = wasmrs_host::Host::new(engine)?;
  let context = host.new_context(64 * 1024, 64 * 1024)?;
  context.dump_operations();

  Ok(())
}
