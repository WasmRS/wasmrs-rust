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
$ cargo run --bin request -- file.wasm NAMESPACE OPERATION 'DATA AS JSON'
```

Req/Stream

```
$ cargo run --bin request -- file.wasm NAMESPACE OPERATION 'DATA AS JSON' --stream
```

Req/Channel

```
$ cargo run --bin request -- file.wasm NAMESPACE OPERATION 'DATA AS JSON' --channel
```

## Enable trace logging

```
RUST_LOG=wasmrs=trace cargo run --bin request ...
```

## All together now

```
$ just debug && RUST_LOG=wasmrs=trace cargo run --bin request -- file.wasm NAMESPACE OPERATION 'DATA AS JSON'
```
