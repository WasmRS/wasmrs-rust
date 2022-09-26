#!/bin/bash

TARGET=wasm32-unknown-unknown

mkdir -p build
# for wasm in target/${TARGET}/release/*.wasm; do
for wasm in target/${TARGET}/release/*.wasm; do
  wasm-opt --strip-debug -Oz $wasm -o build/$(basename $wasm)
done
ls -lh target/${TARGET}/release/*.wasm
ls -lh build/*.wasm
