wasm:
    just wasm/wasmrs-component/build
    ./opt.sh
debug:
    just wasm/wasmrs-component/debug
    ./opt.sh
wit:
    wit-bindgen guest rust --export wasmrs.wit --import wasmrs.wit --out-dir crates/wasmrs/src/
