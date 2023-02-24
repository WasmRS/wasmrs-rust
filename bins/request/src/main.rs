use std::io::{BufRead, Write};

use clap::Parser;
use futures::StreamExt;
use wasmrs::{Metadata, Payload, RSocket};
use wasmrs_codec::messagepack::*;
use wasmrs_host::WasiParams;
use wasmrs_rx::*;
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
  #[arg(default_value = "\"\"")]
  data: String,

  /// The file path to store the frames in a replay file
  #[arg(long = "replay", short = 'r')]
  replay: Option<String>,

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

  if args.channel {
    let stdin = std::io::stdin();
    let (tx, rx) = Flux::new_channels();

    let task = tokio::spawn(async move {
      let mut response = context.request_channel(rx);
      while let Some(Ok(payload)) = response.next().await {
        let bytes = payload.data.unwrap();
        let val: String = deserialize(&bytes).unwrap();
        println!("{}", val);
      }
    });
    for (_i, line) in stdin.lock().lines().enumerate() {
      let bytes = serialize(&line.unwrap()).unwrap();
      let payload = Payload::new(mbytes.clone(), bytes.into());
      let _ = tx.send(payload);
    }
    drop(tx);
    task.await?;
  } else {
    let val: serde_json::Value = serde_json::from_str(&args.data)?;
    let bytes = serialize(&val).unwrap();

    let payload = Payload::new(mbytes, bytes.into());
    if args.stream {
      let mut response = context.request_stream(payload.clone());
      while let Some(Ok(v)) = response.next().await {
        let bytes = v.data.unwrap();
        let val: String = deserialize(&bytes).unwrap();
        println!("{}", val);
      }
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
  }

  if let Some(replay) = args.replay {
    let mut file = std::fs::File::create(replay)?;
    let frames = wasmrs::get_records();
    for frame in frames {
      file.write_fmt(format_args!("{}\n", serde_json::to_string(&frame)?))?;
    }
  }

  Ok(())
}
