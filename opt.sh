#!/bin/bash

mkdir -p build
for wasm in build/req*.wasm; do
  wasm-opt --strip-debug -Oz $wasm -o build/opt.$(basename $wasm)
done
ls -lh build/*.wasm
