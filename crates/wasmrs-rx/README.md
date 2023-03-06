# wasmrs-rx

WasmRS-RX is a simple implementation of rx-like functionality for Rust tailored towards use in wasmrs, the WebAssembly RSocket implementation.

## Note

RX & Reactive Streams revolve around concepts of Observables. This project chooses to retain Flux/Mono terminology to keep it in line with other RSocket implementations.

## Usage

A `Mono` is a single value while a `Flux` is any number of values. They are analogous to Futures and Streams, respectively. In this implementation, each value is either a success or a failure which makes wasmrs-rx's `Mono` and `Flux` feel like an asynchronous `Result` or a stream of `Result`s.

A `Mono` can be instantiated with a single success or failure value as so:

```rs
let mono = Mono::<_, Error>::new_success(100);

let result = mono.await?;

println!("{}", result);
```

It can also be created from a future:

```rs
let mono = Mono::<_, Error>::from_future(async move { Ok(101) });

let result = mono.await?;

println!("{}", result);
```

Or a `Mono` can be created and completed later:

```rs
let mut mono = Mono::<u32, Error>::new();

mono.success(100);

let result = mono.await?;

println!("{}", result);
```

## Flux

A `Flux` is a stream/channel wrapped up together. You can push to it, complete it, and await it:

```rs
let mut flux = FluxChannel::<_, Error>::new();

flux.send(100)?;
flux.send(101)?;
flux.send(102)?;
flux.complete();

while let Some(payload) = flux.next().await {
  println!("{}", payload?);
}
```

You can take the receiver portion and split the send/receive as you would other channels:

```rs
let flux = FluxChannel::<_, Error>::new();
let mut rx = flux.take_rx()?;

let task = tokio::spawn(async move {
  sleep(Duration::from_millis(500)).await;
  flux.send(100).unwrap();
  flux.send(101).unwrap();
  flux.send(102).unwrap();
  flux.complete()
});

while let Some(payload) = rx.next().await {
  println!("{}", payload?);
}
task.await?;
```

Since `Flux`es embed the concept of a `Result`, `.send()` pushes `Ok` values and `.error()` can be used to push error values.

```rs
let mut flux = FluxChannel::<_, Error>::new();

flux.send(100)?;
flux.send(101)?;
flux.send(102)?;
flux.error(anyhow::anyhow!("error"))?;
flux.complete();

while let Some(payload) = flux.next().await {
  println!("{:?}", payload);
}
```

## More Info

For more information on wasmRS, see the core [wasmrs](https://github.com/wasmrs/wasmrs-rust/blob/main/crates/wasmrs/README.md) crate.

WasmRS makes heavy use of generated code from `apex` specs and generators to automate all of the boilerplate. See the [getting-started](https://github.com/WasmRS/docs/blob/main/wasmrs-rust-howto.md) for usage.

## Contributing

See [CONTRIBUTING.md](https://github.com/WasmRS/wasmrs-rust/blob/main/CONTRIBUTING.md)

## License

See the root [LICENSE.txt](https://github.com/WasmRS/wasmrs-rust/blob/main/LICENSE.txt)



