use clap::Parser;
use futures::StreamExt;
use wasmrs::RSocket;
use wasmrs::{Metadata, Payload};
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
    let bytes = serialize(&args.data).unwrap();
    let payload = Payload::new(mbytes, bytes.into());

    if args.stream {
        unimplemented!()
    } else if args.channel {
        unimplemented!()
    } else {
        let mut response = context.request_response(payload.clone());
        match response.next().await {
            Some(Ok(v)) => {
                let val: serde_json::Value = deserialize(&v.data.unwrap()).unwrap();
                println!("{}", val);
            }
            Some(Err(e)) => {
                println!("Error: {}", e)
            }
            None => {
                println!("No response received");
            }
        }
    }

    Ok(())
}
