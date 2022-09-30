use std::time::Instant;

use futures::stream::select_all;
use futures::StreamExt;
use wasmrs::RSocket;
use wasmrs::{Metadata, Payload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeEngineProviderBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let module_bytes = include_bytes!("../../../build/reqres_component.wasm");
    let engine = WasmtimeEngineProviderBuilder::new(module_bytes)
        .wasi_params(WasiParams::default())
        .build()?;
    let host = wasmrs_host::Host::new(engine)?;
    let bytes = serialize("Hello world").unwrap();
    let context = host.new_context()?;
    let start = Instant::now();
    let num = 100000;
    let metadata = Metadata::new("greeting", "sayHello");
    let mbytes = metadata.encode();
    println!("metadata: {:?}", mbytes);
    let payload = Payload::new(mbytes, bytes.into());
    let mut streams = Vec::new();
    for _ in 0..num {
        let stream = context.request_response(payload.clone());
        streams.push(stream);
    }
    let _results: Vec<_> = select_all(streams).collect().await;
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
