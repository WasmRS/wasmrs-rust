wasm:
    mkdir -p build
    just wasm/reqres-component/build
    just wasm/grabbag/build
    just wasm/baseline/build

debug:
    mkdir -p build
    just wasm/reqres-component/debug
    just wasm/grabbag/debug
    just wasm/baseline/debug

test:
  cargo test --workspace

clean:
  cargo clean
  rm -rf build/*
  just wasm/reqres-component/clean
  just wasm/grabbag/clean
  just wasm/baseline/clean