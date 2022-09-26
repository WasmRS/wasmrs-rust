use std::time::Instant;

use wasmflow_codec::messagepack::deserialize;
use wasmrs::{Metadata, Payload};
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeEngineProviderBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let module_bytes =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/wasmrs_component.wasm");
    // let module_bytes = include_bytes!("../../../target/wasm32-wasi/release/wasmrs_component.wasm");
    // let module_bytes = include_bytes!("../../../build/wasmrs_component.wasm");
    let engine = WasmtimeEngineProviderBuilder::new(module_bytes)
        .wasi_params(WasiParams::default())
        .build()?;
    let host = wasmrs_host::Host::new(engine)?;
    let bytes = wasmflow_codec::messagepack::serialize("Hello world").unwrap();
    // let bytes = b"Hello world".to_vec();
    let mut context = host.new_context()?;
    let start = Instant::now();
    let num = 100000;
    let metadata = Metadata::new("greeting", "sayHello");
    let mbytes = metadata.encode();
    println!("metadata: {:?}", mbytes);
    let payload = Payload::new(mbytes, bytes.into());
    for _ in 0..num {
        let result = context.request_response(payload.clone()).await?;
    }
    let end = Instant::now();
    let duration = end - start;
    println!(
        "{} took {} ns ({} ops/ms, {} ns/op)",
        num,
        duration.as_nanos(),
        num / duration.as_millis(),
        duration.as_nanos() / (num as u128)
    );

    Ok(())
}
