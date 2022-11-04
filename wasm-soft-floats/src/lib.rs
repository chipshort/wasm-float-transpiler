//! Software floating point library for WebAssembly.
//!
//! This is a companion crate for `wasm-float-transpiler` that provides the actual implementation
//! of the software floating point operations.
//!
//! Usage is quite simple, just add it as a dependency to your `Cargo.toml` and
//! insert `pub use wasm_soft_floats::*` inside your code to include the softfloat functions.
//! The transpiler can then pick these up and replace the float operations with calls to them.
//! You can optionally choose between different backends for the soft float operations using this crate's features.

pub(crate) mod float;

pub use advanced_ops::*;
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

#[cfg(feature = "softfp")]
mod advanced_ops {
    use super::*;
    use softfp::{F32, F64};

    // special functions needed for the softfp crate
    #[no_mangle]
    fn softfp_get_rounding_mode() -> softfp::RoundingMode {
        softfp::RoundingMode::TiesToEven
    }
    #[no_mangle]
    fn softfp_set_exception_flags(_flags: softfp::ExceptionFlags) {}

    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_ceil(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_floor(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_trunc(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_nearest(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_sqrt(v: u32) -> u32 {
        F32::new(v).square_root().0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_ceil(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_floor(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_trunc(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_nearest(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_sqrt(v: u64) -> u64 {
        F64::new(v).square_root().0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(v: i32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(v: i64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(v: i32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(v: i64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(v: u32) -> i32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_64(v: u64) -> i32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_32(v: u32) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_64(v: u64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_lt(a: u32, b: u32) -> u32 {
        bool(F32::new(a) < F32::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_gt(a: u32, b: u32) -> u32 {
        bool(F32::new(a) > F32::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_le(a: u32, b: u32) -> u32 {
        bool(F32::new(a) <= F32::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_ge(a: u32, b: u32) -> u32 {
        bool(F32::new(a) >= F32::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_lt(a: u64, b: u64) -> u32 {
        bool(F64::new(a) < F64::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_gt(a: u64, b: u64) -> u32 {
        bool(F64::new(a) > F64::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_le(a: u64, b: u64) -> u32 {
        bool(F64::new(a) <= F64::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_ge(a: u64, b: u64) -> u32 {
        bool(F64::new(a) >= F64::new(b))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_add(a: u32, b: u32) -> u32 {
        (F32::new(a) + F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_sub(a: u32, b: u32) -> u32 {
        (F32::new(a) - F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_mul(a: u32, b: u32) -> u32 {
        (F32::new(a) * F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_div(a: u32, b: u32) -> u32 {
        (F32::new(a) / F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_min(a: u32, b: u32) -> u32 {
        F32::min(F32::new(a), F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_max(a: u32, b: u32) -> u32 {
        F32::max(F32::new(a), F32::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_add(a: u64, b: u64) -> u64 {
        (F64::new(a) + F64::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_sub(a: u64, b: u64) -> u64 {
        (F64::new(a) - F64::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_mul(a: u64, b: u64) -> u64 {
        (F64::new(a) * F64::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_div(a: u64, b: u64) -> u64 {
        (F64::new(a) / F64::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_min(a: u64, b: u64) -> u64 {
        F64::min(F64::new(a), F64::new(b)).0
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_max(a: u64, b: u64) -> u64 {
        F64::max(F64::new(a), F64::new(b)).0
    }
}

#[cfg(all(feature = "rustc_apfloat", not(feature = "softfp")))]
mod advanced_ops {
    use super::*;
    use rustc_apfloat::ieee::{Double, Single};
    use rustc_apfloat::Float;

    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_ceil(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_floor(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_trunc(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_nearest(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_sqrt(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_ceil(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_floor(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_trunc(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_nearest(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_sqrt(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(v: i32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(v: i64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(v: u64) -> u32 {
        let res = Single::from_u128(v as u128);
        res.value.to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(v: i32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(v: i64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(v: u32) -> i32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_32(v: u32) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_64(v: u64) -> i32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_64(v: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_32(v: u32) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_32(v: u32) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_64(v: u64) -> i64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_64(v: u64) -> u64 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_lt(a: u32, b: u32) -> u32 {
        bool(Single::from_bits(a as u128) < Single::from_bits(b as u128))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_gt(a: u32, b: u32) -> u32 {
        bool(Single::from_bits(a as u128) > Single::from_bits(b as u128))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_le(a: u32, b: u32) -> u32 {
        bool(Single::from_bits(a as u128) <= Single::from_bits(b as u128))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_ge(a: u32, b: u32) -> u32 {
        bool(Single::from_bits(a as u128) >= Single::from_bits(b as u128))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_lt(a: u64, b: u64) -> u32 {
        bool(Double::from_bits(a as u128) < Double::from_bits(b as u128))
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_gt(a: u64, b: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_le(a: u64, b: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_ge(a: u64, b: u64) -> u32 {
        todo!()
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_add(a: u32, b: u32) -> u32 {
        let res = Single::from_bits(a as u128) + Single::from_bits(b as u128);
        res.value.to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_sub(a: u32, b: u32) -> u32 {
        let res = Single::from_bits(a as u128) - Single::from_bits(b as u128);
        res.value.to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_mul(a: u32, b: u32) -> u32 {
        let res = Single::from_bits(a as u128) * Single::from_bits(b as u128);
        res.value.to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_div(a: u32, b: u32) -> u32 {
        let res = Single::from_bits(a as u128) / Single::from_bits(b as u128);
        res.value.to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_min(a: u32, b: u32) -> u32 {
        Single::from_bits(a as u128)
            .min(Single::from_bits(b as u128))
            .to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_32_max(a: u32, b: u32) -> u32 {
        Single::from_bits(a as u128)
            .max(Single::from_bits(b as u128))
            .to_bits() as u32
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_add(a: u64, b: u64) -> u64 {
        let res = Double::from_bits(a as u128) + Double::from_bits(b as u128);
        res.value.to_bits() as u64
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_sub(a: u64, b: u64) -> u64 {
        let res = Double::from_bits(a as u128) - Double::from_bits(b as u128);
        res.value.to_bits() as u64
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_mul(a: u64, b: u64) -> u64 {
        let res = Double::from_bits(a as u128) * Double::from_bits(b as u128);
        res.value.to_bits() as u64
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_div(a: u64, b: u64) -> u64 {
        let res = Double::from_bits(a as u128) / Double::from_bits(b as u128);
        res.value.to_bits() as u64
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_min(a: u64, b: u64) -> u64 {
        Double::from_bits(a as u128)
            .min(Double::from_bits(b as u128))
            .to_bits() as u64
    }
    #[no_mangle]
    pub extern "C" fn __wasm_soft_float_f_64_max(a: u64, b: u64) -> u64 {
        Double::from_bits(a as u128)
            .max(Double::from_bits(b as u128))
            .to_bits() as u64
    }
}

#[inline(always)]
fn bool(v: bool) -> u32 {
    if v {
        1
    } else {
        0
    }
}

// reinterpret instructions are handled as noop in the transpiler
// load and store instructions are converted to integer loads / stores in the transpiler
