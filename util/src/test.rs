//! Some helpers to test a backend against the wasm spec

use crate::float::*;
use crate::*;

#[macro_export]
macro_rules! impl_test_fn {
    ($name:ident ($($arg:ident : $t:ty),*)) => {
        quickcheck::quickcheck! {
            fn $name($($arg: $t),*) -> bool {
                $crate::test::$name(BINOPS32, $($arg),*)
            }
        }
    };
}

/// Implements the test suite for a backend
/// In order to use this, you need to have a `quickcheck` dev-dependency.
/// Also make sure to have the `__wasm_soft_float_*` functions in scope.
#[doc(hidden)]
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! impl_tests {
    () => {
        #[cfg(test)]
        mod tests {
            use super::*;
            const BINOPS32: $crate::test::Binops32 = $crate::test::Binops32 {
                __wasm_soft_float_f_32_add,
                __wasm_soft_float_f_32_sub,
            };

            impl_test_fn!(add_neg_is_zero(a: u32));
            impl_test_fn!(sub_is_add_neg(a: u32, b: u32));
        }
    };
}

pub struct Binops32 {
    pub __wasm_soft_float_f_32_add: Binop32,
    pub __wasm_soft_float_f_32_sub: Binop32,
}

pub type Binop32 = extern "C" fn(u32, u32) -> u32;

pub fn add_neg_is_zero(binops: Binops32, a: u32) -> bool {
    let fa = F32::from_bits(a);
    fa.is_infinite()
        || fa.is_nan()
        || (binops.__wasm_soft_float_f_32_add)(a, __wasm_soft_float_f_32_neg(a)) == 0f32.to_bits()
}

pub fn sub_is_add_neg(binops: Binops32, a: u32, b: u32) -> bool {
    f32::from_bits(a).is_nan()
        || f32::from_bits(b).is_nan()
        || (binops.__wasm_soft_float_f_32_sub)(a, b)
            == (binops.__wasm_soft_float_f_32_add)(a, __wasm_soft_float_f_32_neg(b))
}
