# Wasm Float Transpiler

Converts a `wasm` file with floating point instructions (`f32`, `f64`) to one without such instructions.
The instructions are replaced with deterministic software implementations of these instructions.

# Usage

In order to include the softfloat implementation in your project,
add the `wasm-soft-floats` crate as a dependency and include it in your build by adding a
`pub use wasm_soft_floats::*;` to your project.

Then compile your project and call `wasm-float-transpiler` on your `wasm` file, like this:
```bash
wasm-float-transpiler my_project.wasm output.wasm
```
The resulting `output.wasm` file now contains your finished WebAssembly without any trace of floating
point operations and with only the softfloat functions that you actually use.