use std::time::Instant;

use rxrust::prelude::*;
use wasmrs_host::{OutgoingStream, WasiParams, WasmRsHostBuilder};
use wasmrs_wasmtime::WasmtimeEngineProviderBuilder;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    // let module_bytes =
    //     include_bytes!("../../../target/wasm32-unknown-unknown/release/wasmrs_component.wasm");
    let module_bytes = include_bytes!("../../../build/wasmrs_component.wasm");
    let engine = WasmtimeEngineProviderBuilder::new(module_bytes)
        .wasi_params(WasiParams::default())
        .build()?;
    let host = WasmRsHostBuilder::new().build(engine)?;
    let bytes = wasmflow_codec::messagepack::serialize("Hello world").unwrap();
    let mut context = host.new_context()?;
    let start = Instant::now();
    let num = 10000;
    for _ in 0..num {
        let stream = context.request_response("greeting", "sayHello", bytes.clone())?;
        stream
            .clone()
            .subscribe(|v| println!("got something {:?}", v));
        // stream.clone().map(|_v| println!("yay"));
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
