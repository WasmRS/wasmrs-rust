# wasmrs

WasmRS is an implementation of Reactive Streams for WebAssembly modules that allows hosts & guests to communicate via asynchronous, bidirectional streams.

The `wasmrs` crate is the base implementation of the bidirectional WebAssembly socket.

## Usage

See [wasmrs-guest](https://github.com/wasmrs/wasmrs-rust/blob/main/crates/wasmrs-guest/README.md), [wasmrs-host](https://github.com/wasmrs/wasmrs-rust/blob/main/crates/wasmrs-guest/README.md), and [wasmrs-wamtime](https://github.com/wasmrs/wasmrs-rust/blob/main/crates/wasmrs-guest/README.md) for examples on how to use wasmrs directly.

## More Information

For more information on wasmRS, see the core [wasmrs](https://github.com/wasmrs/wasmrs-rust/blob/main/crates/wasmrs/README.md) crate.

WasmRS makes heavy use of generated code from `apex` specs and generators to automate all of the boilerplate. See the [getting-started](https://github.com/WasmRS/docs/blob/main/wasmrs-rust-howto.md) for usage.

## Contributing

See [CONTRIBUTING.md](https://github.com/WasmRS/wasmrs-rust/blob/main/CONTRIBUTING.md)

## License

See the root [LICENSE.txt](https://github.com/WasmRS/wasmrs-rust/blob/main/LICENSE.txt)



