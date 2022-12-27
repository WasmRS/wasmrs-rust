# WasmRS

WasmRS is an implementation of [reactive streams](https://www.reactive-streams.org) in WebAssembly modules to enable asynchronous, bidirectional communication in and out of wasm. WasmRS is a spiritual successor to waPC and expands on what you can do with WebAssembly dramatically.

## wasmRS Protocol

WasmRS revolves around a handful of methods that allow the host and the guest to write [RSocket](https://rsocket.io) frames to their respective buffers in WebAssembly memory. The language-specific implementation largely handles the encoding and decoding of these frames with a light UX layer on top and metadata extensions that are relevant to WebAssembly usage.

Just as in RSocket, wasmRS frames contain a stream ID that allow the destination to differentiate multiple frames for different transactions.

For more information on the protocol, see the [wasmRS documentation](https://github.com/nanobus/iota/blob/main/docs/wasmrs.md) in the root of this project.

## PreRequisites

- [just](github.com/casey/just) task runner.

## Building & Running tests

The host tests depend on built WebAssembly modules. To build new modules, run:

```sh
$ just wasm
```

Build debug versions of the WebAssembly modules (with `wasi`) using:

```sh
$ just debug
```

Run tests with the command `just test`:

```sh
$ just test
```

## Running example host with guest wasm

The `request` binary allows you to make simple requests into WebAssembly binaries, passing JSONified data as input, e.g.:

```
$ cargo run --bin request -- ./build/reqres_component.wasm suite.test reverse '{"input":"abcdefghijklmnopqrstuvwxyz"}'
```

## Enable trace logging

```
RUST_LOG=wasmrs=trace cargo run --bin request ...
```

## See also

- [nanobus](https://github.com/nanobus/nanobus) as a way to run wasmRS modules
- [apex](https://apexlang.io) to generate boilerplate for iotas and projects using wasmrs.

## Contributing

See [CONTRIBUTING.md](https://github.com/nanobus/iota/blob/main/CONTRIBUTING.md)

## License

See the root [LICENSE.txt](https://github.com/nanobus/iota/blob/main/LICENSE.txt)



