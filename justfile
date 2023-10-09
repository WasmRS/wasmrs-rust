# Apex-oriented projects have gone stale but can still
# be used as regression tests.

wasm:
    mkdir -p build
    just wasm/baseline/build

debug:
    mkdir -p build
    just wasm/baseline/debug

test:
  cargo test --workspace
  cargo test -p wasmrs-runtime --target=wasm32-unknown-unknown

clean:
  cargo clean
  rm -rf build/*
  just wasm/baseline/clean