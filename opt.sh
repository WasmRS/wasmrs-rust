#!/bin/bash

mkdir -p build
for wasm in wasm/*/target/wasm32-unknown-unknown/release/*.wasm; do
  wasm-opt --strip-debug -Oz $wasm -o build/$(basename $wasm)
done
