#!/bin/bash

mkdir -p build
# for wasm in target/wasm32-wasi/release/*.wasm; do
for wasm in target/wasm32-unknown-unknown/release/*.wasm; do
  wasm-opt --strip-debug -Oz $wasm -o build/$(basename $wasm)
done
ls -lh target/wasm32-wasi/release/*.wasm
ls -lh build/*.wasm
