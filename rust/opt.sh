#!/bin/bash

for wasm in build/req*.wasm; do
  wasm-opt --strip-debug -Oz $wasm -o build/opt.$(basename $wasm)
done
