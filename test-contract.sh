#!/usr/bin/env sh

# Compiles cw-float and runs the transpiler on it.
# The ouput is stored in output.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --release --lib --target wasm32-unknown-unknown -p cw-float --locked
cargo run --bin wasm-float-transpiler -- ./target/wasm32-unknown-unknown/release/cw_float.wasm output.wasm