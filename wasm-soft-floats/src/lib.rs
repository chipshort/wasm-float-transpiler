//! Software floating point library for WebAssembly.
//!
//! This is a companion crate for `wasm-float-transpiler` that provides the actual implementation
//! of the software floating point operations.
//!
//! Usage is quite simple, just add it as a dependency to your `Cargo.toml` and
//! insert `pub use wasm_soft_floats::*` inside your code to include the softfloat functions.
//! The transpiler can then pick these up and replace the float operations with calls to them.
//! You can optionally choose between different backends for the soft float operations using this crate's features.

mod backend;
pub(crate) mod float;

pub use backend::*;
pub use simple_ops::*;

mod simple_ops {
    use super::*;
    use core::ops::Neg;
    use float::{F32, F64};

    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_reinterpret_f_32(v: u32) -> u32 {
        v
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_reinterpret_f_64(v: u64) -> u64 {
        v
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_reinterpret_i_32(v: u32) -> u32 {
        v
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_reinterpret_i_64(v: u64) -> u64 {
        v
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_abs(v: u32) -> u32 {
        F32::from_bits(v).abs().to_bits()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_neg(v: u32) -> u32 {
        F32::from_bits(v).neg().to_bits()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_eq(a: u32, b: u32) -> u32 {
        bool(F32::from_bits(a) == F32::from_bits(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_ne(a: u32, b: u32) -> u32 {
        bool(F32::from_bits(a) != F32::from_bits(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_copysign(a: u32, b: u32) -> u32 {
        F32::from_bits(a).copy_sign(F32::from_bits(b)).to_bits()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_abs(v: u64) -> u64 {
        F64::from_bits(v).abs().to_bits()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_neg(v: u64) -> u64 {
        F64::from_bits(v).neg().to_bits()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_eq(a: u64, b: u64) -> u32 {
        bool(F64::from_bits(a) == F64::from_bits(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_ne(a: u64, b: u64) -> u32 {
        bool(F64::from_bits(a) != F64::from_bits(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_copysign(a: u64, b: u64) -> u64 {
        F64::from_bits(a).copy_sign(F64::from_bits(b)).to_bits()
    }
}

#[inline(always)]
pub(crate) fn bool(v: bool) -> u32 {
    u32::from(v)
}

// reinterpret instructions are handled as noop in the transpiler
// load and store instructions are converted to integer loads / stores in the transpiler
