# wasmrs-guest

This crate provides the WebAssembly-side logic for wasmRS modules using the wasmRS RSocket protocol.


## Usage

This is a basic implementation of a WebAssembly module that exports three operations:

- `greeting::sayHello(input: string) -> string` - returns a greeting, e.g. `Hello World!'
- `echo::chars(input: string) -> stream string` - returns a stream of `string` representing each character in the input string
- `echo::reverse(input: stream string) -> stream string` - reverses each `string` from the input stream and outputs it on a stream.

```rs
use guest::*;
use wasmrs_guest as guest;

#[no_mangle]
extern "C" fn __wasmrs_init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);

  guest::register_request_response("greeting", "sayHello", request_response);
  guest::register_request_stream("echo", "chars", request_stream);
  guest::register_request_channel("echo", "reverse", request_channel);
}

fn request_response(input: Mono<ParsedPayload, PayloadError>) -> Result<Mono<Payload, PayloadError>, GenericError> {
  Ok(Mono::from_future(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    let output = format!("Hello, {}!", input);
    Ok(Payload::new_data(None, Some(serialize(&output).unwrap().into())))
  }))
}

fn request_stream(
  input: Mono<ParsedPayload, PayloadError>,
) -> Result<FluxReceiver<Payload, PayloadError>, GenericError> {
  let channel = Flux::<Payload, PayloadError>::new();
  let rx = channel.take_rx().unwrap();
  spawn(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    for char in input.chars() {
      channel
        .send(Payload::new_data(None, Some(serialize(&char).unwrap().into())))
        .unwrap();
    }
  });

  Ok(rx)
}
fn request_channel(
  mut input: FluxReceiver<ParsedPayload, PayloadError>,
) -> Result<FluxReceiver<Payload, PayloadError>, GenericError> {
  let channel = Flux::<Payload, PayloadError>::new();
  let rx = channel.take_rx().unwrap();
  spawn(async move {
    while let Some(payload) = input.next().await {
      if let Err(e) = payload {
        println!("{}", e);
        continue;
      }
      let payload = payload.unwrap();
      let input = deserialize::<String>(&payload.data).unwrap();
      let output: String = input.chars().rev().collect();
      if let Err(e) = channel.send(Payload::new_data(None, Some(serialize(&output).unwrap().into()))) {
        println!("{}", e);
      }
    }
  });

  Ok(rx)
}
```

## Apex Code generators

NanoBus iota code generators use the wasmRS protocol. You can build `wasmRS` modules from those templates using the [`https://github.com/apexlang/apex`](apex) CLI.

Run the following command to get started:

```sh
$ apex new git@github.com:nanobus/iota.git -p templates/rust [your-project]
```

From there, edit the `apex.axdl` interface definition to match your needs and run `apex build` to generate the wasmRS module.

## More Information

WasmRS makes heavy use of generated code from `apex` specs and generators to automate all of the boilerplate. See the [getting-started](https://github.com/nanobus/nanobus/blob/main/docs/getting-started.md) for NanoBus for up-to-date usage.

For more information on wasmRS, see the core [wasmrs](https://github.com/nanobus/iota/blob/main/rust/crates/wasmrs/README.md) crate.

## Contributing

See [CONTRIBUTING.md](https://github.com/nanobus/iota/blob/main/CONTRIBUTING.md)

## License

See the root [LICENSE.txt](https://github.com/nanobus/iota/blob/main/LICENSE.txt)



