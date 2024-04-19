#!/bin/bash

cargo run --target wasm32-unknown-unknown --all-features

# Check if cargo run was successful
if [ $? -eq 0 ]; then
    wasm-server-runner ./target/wasm32-unknown-unknown/debug/rust_jumpnrun.wasm
fi
