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

```
$ cargo run -p wasmrs-wasmtime --example wasmrs --release
```
