# wasmrs-runtime

wasmrs-runtime is a set of structs and functions that are normalized across multithreaded native tokio and single-threaded WebAssembly using *whatever the smallest, fastest, single threaded, async, WebAssembly-compatible runtime of the day is*.

## Notice!

You're better off not relying on this crate. It's a crate that exists only for as long as it needs to. As WebAssembly matures and there are more standard solutions for the problems this crate solves, this crate will be deprecated.

## More Info

For more information on wasmRS, see the core [wasmrs](https://github.com/nanobus/iota/blob/main/rust/crates/wasmrs/README.md) crate.

WasmRS makes heavy use of generated code from `apex` specs and generators to automate all of the boilerplate. See the [getting-started](https://github.com/nanobus/nanobus/blob/main/docs/getting-started.md) for nanobus for up-to-date usage.

## Contributing

See [CONTRIBUTING.md](https://github.com/nanobus/iota/blob/main/CONTRIBUTING.md)

## License

See the root [LICENSE.txt](https://github.com/nanobus/iota/blob/main/LICENSE.txt)



