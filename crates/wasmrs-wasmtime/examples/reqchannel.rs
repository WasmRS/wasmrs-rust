use std::time::Instant;

use wasmrs::flux::{Flux, Observer};
use wasmrs::RSocket;
use wasmrs::{Metadata, Payload};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_wasmtime::WasmtimeEngineProviderBuilder;

use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let module_bytes = include_bytes!("../../../build/req_channel_component.wasm");
    let engine = WasmtimeEngineProviderBuilder::new(module_bytes)
        .wasi_params(WasiParams::default())
        .build()?;
    let host = wasmrs_host::Host::new(engine)?;
    let bytes = serialize("Hello world").unwrap();
    let context = host.new_context()?;
    let start = Instant::now();
    let num = 1;
    let metadata = Metadata::new("greeting", "sayHello");
    let mbytes = metadata.encode();
    println!("metadata: {:?}", mbytes);
    let payload = Payload::new(mbytes, bytes.into());
    for _ in 0..num {
        println!("Making request stream");
        let flux = Flux::new();

        let mut stream = context.request_channel(flux.split_receiver()?);
        flux.send(payload.clone());
        flux.send(payload.clone());
        flux.send(payload.clone());
        flux.send(payload.clone());
        flux.complete();
        println!("Request returned");

        while let Some(payload) = stream.next().await {
            println!("Got payload: {:?}", payload);
            match payload {
                Ok(p) => {
                    if let Some(data) = p.data {
                        // println!("data=: {:?}", data.to_vec());
                        let str: String = deserialize(&data)?;
                        println!("Got value: {}", str);
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        println!("done with stream");
    }
    let end = Instant::now();
    let duration = end - start;
    // println!(
    //     "{} took {} ns ({} ops/ms, {} ms/op)",
    //     num,
    //     duration.as_nanos(),
    //     num / duration.as_millis(),
    //     duration.as_millis() / (num as u128)
    // );

    Ok(())
}
