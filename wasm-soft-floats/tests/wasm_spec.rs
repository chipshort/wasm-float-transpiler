// TODO: add whole spec as tests

use quickcheck::quickcheck;
use wasm_soft_floats::*;

quickcheck! {
    fn negative_neg_is_abs(f: u32) -> bool {
        f32::from_bits(f).is_sign_positive() || __wasm_soft_float_f_32_neg(f) == __wasm_soft_float_f_32_abs(f)
    }

    fn add_works(a: f32, b: f32) -> bool {
        __wasm_soft_float_f_32_add(a.to_bits(), b.to_bits()) == (a + b).to_bits()
    }

    fn add_neg_is_zero(a: u32) -> bool {
        let fa = f32::from_bits(a);
        fa.is_infinite() || fa.is_nan() || __wasm_soft_float_f_32_add(a, __wasm_soft_float_f_32_neg(a)) == 0f32.to_bits()
    }

    fn sub_works(a: f32, b: f32) -> bool {
        // (a, b) = (0.0, NaN) results in different NaNs on my machine
        a.is_nan() || b.is_nan() || __wasm_soft_float_f_32_sub(a.to_bits(), b.to_bits()) == (a - b).to_bits()
    }

    fn sub_is_add_neg(a: u32, b: u32) -> bool {
        f32::from_bits(a).is_nan() || f32::from_bits(b).is_nan() || __wasm_soft_float_f_32_sub(a, b) == __wasm_soft_float_f_32_add(a, __wasm_soft_float_f_32_neg(b))
    }

    fn mul_works(a: f32, b: f32) -> bool {
        __wasm_soft_float_f_32_mul(a.to_bits(), b.to_bits()) == (a * b).to_bits()
    }
}
