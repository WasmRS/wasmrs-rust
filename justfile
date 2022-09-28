wasm:
    just wasm/reqres-component/build
    just wasm/req-stream-component/build
    ./opt.sh
debug:
    just wasm/reqres-component/debug
    just wasm/req-stream-component/debug
    ./opt.sh
