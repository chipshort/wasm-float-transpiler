pub use wasm_soft_float_bs::*;

#[no_mangle]
pub extern "C" fn test(a: f32, b: f32) -> f32 {
    a + b
}
