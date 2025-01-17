#!/bin/bash

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/mining-poc.wasm --out-dir ./out --no-typescript
