wasm:
    just wasm/reqres-component/build
    just wasm/req-stream-component/build
    just wasm/req-channel-component/build
    ./opt.sh
debug:
    just wasm/reqres-component/debug
    just wasm/req-stream-component/debug
    just wasm/req-channel-component/debug
    # ./opt.sh
