use std::time::Instant;

use wasmrs::{Metadata, Payload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeEngineProviderBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let module_bytes = include_bytes!("../../../build/wasmrs_component.wasm");
    // let module_bytes = include_bytes!("../../../target/wasm32-wasi/release/wasmrs_component.wasm");
    // let module_bytes = include_bytes!("../../../build/wasmrs_component.wasm");
    let engine = WasmtimeEngineProviderBuilder::new(module_bytes)
        .wasi_params(WasiParams::default())
        .build()?;
    let host = wasmrs_host::Host::new(engine)?;
    let bytes = serialize("Hello world").unwrap();
    // let bytes = b"Hello world".to_vec();
    let mut context = host.new_context()?;
    let start = Instant::now();
    let num = 100000;
    let metadata = Metadata::new("greeting", "sayHello");
    let mbytes = metadata.encode();
    println!("metadata: {:?}", mbytes);
    let payload = Payload::new(mbytes, bytes.into());
    for _ in 0..num {
        let _result = context.request_response(payload.clone()).await?;

        // if let Some(data) = result.and_then(|v| v.data) {
        //     let str: String = deserialize(&data)?;
        //     println!("Got: {}", str);
        // }
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
