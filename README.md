# Wasm Float Transpiler

Converts a `wasm` file with floating point instructions (`f32`, `f64`) to one without such instructions.
The instructions are replaced with deterministic software implementations of these instructions.

# Usage

In order to include the softfloat implementation in your project,
choose one of the `wasm-soft-float-*` backend crates as a dependency and include it in your build by adding a
`pub use wasm_soft_floats_*::*;` to your project.

Check out the [backends](./backends) folder for the complete set of backend options and their supported operations.
Your best bet is probably `wasm-soft-float-bs` based on the Berkeley Softfloat library. It covers almost all instructions.

Example `lib.rs`:
```rust
pub use wasm_soft_float_bs::*;

#[no_mangle]
pub extern "C" fn test(a: f32, b: f32) -> f32 {
    a + b
}
```

Then compile your project and call `wasm-float-transpiler` on your `wasm` file, like this:
```bash
wasm-float-transpiler my_project.wasm output.wasm
```
The resulting `output.wasm` file now contains your finished WebAssembly without any trace of floating
point operations and with only the softfloat functions that you actually use.

# Example

Check out the [examples](./examples) folder.
To compile the examples run:
```bash
cd examples
cargo build --target wasm32-unknown-unknown --release --workspace
cd ..
```
To then transpile the resulting wasm, run:
```bash
cargo run -p wasm-float-transpiler -- ./examples/target/wasm32-unknown-unknown/release/basic_wasm_float.wasm output.wasm
```