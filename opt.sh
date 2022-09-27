#!/bin/bash

mkdir -p build
wasm-opt --strip-debug -Oz build/wasmrs_component.wasm -o build/wasmrs_component.opt.wasm
ls -lh build/*.wasm
