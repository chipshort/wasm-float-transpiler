// TODO: add whole spec as tests

use quickcheck::quickcheck;
use wasm_soft_floats::import_soft_floats;

import_soft_floats!();

quickcheck! {
    fn negative_neg_is_abs(f: u32) -> bool {
        BaseF32::from_bits(f).is_sign_positive() || __wasm_soft_float_f_32_neg(f) == __wasm_soft_float_f_32_abs(f)
    }

    fn add_works(a: f32, b: f32) -> bool {
        __wasm_soft_float_f_32_add(a.to_bits(), b.to_bits()) == (a + b).to_bits()
    }

    fn add_neg_is_zero(a: u32) -> bool {
        let sa = BaseF32::from_bits(a);
        sa.is_infinity() || sa.is_nan() || __wasm_soft_float_f_32_add(a, __wasm_soft_float_f_32_neg(a)) == BaseF32::ZERO.to_bits()
    }

    fn sub_works(a: f32, b: f32) -> bool {
        let sa = BaseF32::from_bits(a.to_bits());
        let sb = BaseF32::from_bits(b.to_bits());
        // (a, b) = (0.0, NaN) results in different NaNs on my machine
        sa.is_nan() || sb.is_nan() || __wasm_soft_float_f_32_sub(a.to_bits(), b.to_bits()) == (a - b).to_bits()
    }

    fn sub_is_add_neg(a: u32, b: u32) -> bool {
        let sa = BaseF32::from_bits(a);
        let sb = BaseF32::from_bits(b);

        sa.is_nan() || sb.is_nan() || __wasm_soft_float_f_32_sub(a, b) == __wasm_soft_float_f_32_add(a, __wasm_soft_float_f_32_neg(b))
    }

    fn mul_works(a: f32, b: f32) -> bool {
        __wasm_soft_float_f_32_mul(a.to_bits(), b.to_bits()) == (a * b).to_bits()
    }
}
