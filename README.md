# WIP

## PreReqs

- just
- wasm-opt

## Running tests

```
$ cargo test --workspace
```

## Building guest wasm

Release version:

```
$ just
```

Debug version w/wasi:

```
$ just debug
```

## Running example host with guest wasm

Req/Response

```
$ cargo run -p wasmrs-wasmtime --example wasmrs --release
```

Req/Stream

```
$ cargo run -p wasmrs-wasmtime --example reqstream
```

Req/Channel

```
$ cargo run -p wasmrs-wasmtime --example reqchannel
```

## Enable trace logging

```
RUST_LOG=wasmrs=trace cargo run -p wasmrs-wasmtime --example reqstream
```

## All together now

```
$ just debug && RUST_LOG=wasmrs=trace cargo run -p wasmrs-wasmtime --example reqstream
```
