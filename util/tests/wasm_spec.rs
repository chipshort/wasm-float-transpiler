// TODO: add whole spec as tests

use quickcheck::quickcheck;
use wasm_soft_float_utils::{float::*, *};

quickcheck! {
    fn negative_neg_is_abs(f: u32) -> bool {
        F32::from_bits(f).is_sign_positive() || __wasm_soft_float_f_32_neg(f) == __wasm_soft_float_f_32_abs(f)
    }
}
