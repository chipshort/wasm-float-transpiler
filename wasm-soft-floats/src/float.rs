//! Software emulated floating point operations.
//!
//! https://webassembly.github.io/spec/core/exec/numerics.html#floating-point-operations
//! # Rounding
//! round-to-nearest ties-to-even
//! # NaN propagation
//! According to WebAssembly spec, the sign and payload are non-deterministic,
//! but this crate will always return the same result for the same input,
//! regardless of CPU architecture / instruction set.

use core::ops::{Add, Div, Mul, Neg, Sub};

use rustc_apfloat::Float;

trait Test {
    fn test();
}

macro_rules! impl_float {
    ($name: ident, $bits: ident, $exp_bits: tt) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name(pub(crate) $bits);

        impl $name {
            const BIT_SIZE: usize = core::mem::size_of::<$bits>() * 8;
            const EXP_BITS: usize = $exp_bits;
            const MANTISSA_BITS: usize = Self::BIT_SIZE - Self::EXP_BITS - 1;

            const SIGN_MASK: $bits = 1 << (Self::BIT_SIZE - 1);
            const FRAC_MASK: $bits = $bits::MAX >> ($exp_bits + 1);
            const EXP_MASK: $bits = !(Self::SIGN_MASK | Self::FRAC_MASK);
            /// The canonical NaN value.
            /// Please keep in mind that there are other NaN values, so-called arithmetic NaNs.
            /// See https://webassembly.github.io/spec/core/syntax/values.html#syntax-float
            pub const NAN: Self = Self::from_bits(Self::EXP_MASK | 1 << (Self::MANTISSA_BITS - 1));
            pub const NEG_NAN: Self = Self::from_bits(Self::NAN.0 | Self::SIGN_MASK);
            pub const INFINITY: Self = Self::from_bits(Self::EXP_MASK);
            pub const NEG_INFINITY: Self = Self::from_bits(Self::EXP_MASK | Self::SIGN_MASK);

            pub const ONE: Self = Self::from_bits(Self::EXP_MASK ^ (1 << (Self::BIT_SIZE - 2)));
            pub const NEG_ONE: Self = Self::from_bits(Self::ONE.0 | Self::SIGN_MASK);
            pub const ZERO: Self = Self::from_bits(0);
            pub const NEG_ZERO: Self = Self::from_bits(Self::SIGN_MASK);

            pub(crate) const fn exponent(self) -> $bits {
                self.0 & Self::EXP_MASK
            }

            pub(crate) const fn fraction(self) -> $bits {
                self.0 & Self::FRAC_MASK
            }

            pub const fn is_sign_positive(self) -> bool {
                self.0 & Self::SIGN_MASK == 0
            }

            /// Returns `self`, but with the sign bit set to `other`'s sign bit.
            ///
            pub const fn copy_sign(self, other: Self) -> Self {
                Self((self.0 & !Self::SIGN_MASK) | (other.0 & Self::SIGN_MASK))
            }

            /// Returns true if `self` is some kind of NaN.
            pub const fn is_nan(self) -> bool {
                self.exponent() == Self::EXP_MASK && self.fraction() != 0
            }

            /// Returns true if `self` is positive or negative infinity.
            pub const fn is_infinity(self) -> bool {
                (self.0 & (Self::EXP_MASK | Self::FRAC_MASK)) == Self::EXP_MASK
            }

            /// Returns true if `self` is positive or negative zero.
            pub const fn is_zero(self) -> bool {
                self.0 & Self::EXP_MASK == 0
            }

            /// Computes the absolute value of `self`.
            ///
            /// This just clears the sign bit.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fabs
            pub const fn abs(self) -> Self {
                Self(self.0 & !Self::SIGN_MASK)
            }

            /// Reinterprets the given bits as this type.
            ///
            /// This is a no-op.
            pub const fn from_bits(bits: $bits) -> Self {
                Self(bits)
            }

            /// Returns the bit representation of this type.
            ///
            /// This is a no-op.
            pub const fn to_bits(self) -> $bits {
                self.0
            }
        }

        impl Neg for $name {
            type Output = Self;

            /// Computes the negation of `self`.
            ///
            /// This just flips the sign bit.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fneg
            fn neg(self) -> Self::Output {
                Self(self.0 ^ Self::SIGN_MASK)
            }
        }

        impl PartialEq for $name {
            /// Returns `false` if either value is `NaN`, `true` if both are zeroes and
            /// `true` if both have the same value. `false` otherwise
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-feq
            fn eq(&self, other: &Self) -> bool {
                if self.is_nan() || other.is_nan() {
                    false
                } else if self.is_zero() && other.is_zero() {
                    true
                } else {
                    self.0 == other.0
                }
            }
        }

        impl Add for $name {
            type Output = Self;

            /// Adds `self` to `rhs` and returns the result.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fadd
            fn add(self, rhs: Self) -> Self::Output {
                // just calling rustc_apfloat for now, because it's easier
                let a = rustc_apfloat::ieee::Single::from_bits(self.0 as u128);
                let b = rustc_apfloat::ieee::Single::from_bits(rhs.0 as u128);

                let result = a + b;
                Self(rustc_apfloat::ieee::Semantics::to_bits(result.value) as $bits)
            }
        }

        impl Sub for $name {
            type Output = Self;

            /// Subtracts `rhs` from `self` and returns the result.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fsub
            fn sub(self, rhs: Self) -> Self::Output {
                // just calling rustc_apfloat for now, because it's easier
                let a = rustc_apfloat::ieee::Single::from_bits(self.0 as u128);
                let b = rustc_apfloat::ieee::Single::from_bits(rhs.0 as u128);

                let result = a - b;
                Self(rustc_apfloat::ieee::Semantics::to_bits(result.value) as $bits)
            }
        }

        impl Mul for $name {
            type Output = Self;

            /// Multiplies `self` by `rhs` and returns the result.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fmul
            fn mul(self, rhs: Self) -> Self::Output {
                // just calling rustc_apfloat for now, because it's easier
                let a = rustc_apfloat::ieee::Single::from_bits(self.0 as u128);
                let b = rustc_apfloat::ieee::Single::from_bits(rhs.0 as u128);

                let result = a * b;
                Self(rustc_apfloat::ieee::Semantics::to_bits(result.value) as $bits)
            }
        }

        impl Div for $name {
            type Output = Self;

            /// Divides `self` by `rhs` and returns the result.
            ///
            /// https://webassembly.github.io/spec/core/exec/numerics.html#op-fdiv
            fn div(self, rhs: Self) -> Self::Output {
                // just calling rustc_apfloat for now, because it's easier
                let a = rustc_apfloat::ieee::Single::from_bits(self.0 as u128);
                let b = rustc_apfloat::ieee::Single::from_bits(rhs.0 as u128);

                let result = a / b;
                Self(rustc_apfloat::ieee::Semantics::to_bits(result.value) as $bits)
            }
        }
    };
}

impl_float!(F32, u32, 8);
impl_float!(F64, u64, 11);

#[cfg(test)]
mod test {
    use super::*;
    use core::ops::Neg;
    use quickcheck::quickcheck;

    quickcheck! {
        fn sign_works(f: f32) -> bool {
            let sf = F32(f.to_bits());

            sf.is_sign_positive() == f.is_sign_positive()
        }

        fn abs_works(f: f32) -> bool {
            let sf = F32(f.to_bits());

            sf.abs().0 == f.abs().to_bits()
        }

        fn neg_works(f: f32) -> bool {
            let sf = F32(f.to_bits());

            sf.neg().0 == f.neg().to_bits()
        }

        fn copy_sign_works(f: f32, g: f32) -> bool {
            let sf = F32(f.to_bits());
            let sg = F32(g.to_bits());

            let same_sign = sf.is_sign_positive() == sg.is_sign_positive();
            if same_sign {
                sf.copy_sign(sg).0 == sf.0
            } else {
                sf.copy_sign(sg).0 == sf.neg().0
            }
        }

        fn negative_neg_is_abs(f: u32) -> bool {
            let f = F32(f);

            f.is_sign_positive() || f.neg().0 == f.abs().0
        }

        fn add_works(a: f32, b: f32) -> bool {
            let sa = F32(a.to_bits());
            let sb = F32(b.to_bits());

            (sa + sb).0 == (a + b).to_bits()
        }

        fn add_neg_is_zero(a: u32) -> bool {
            let a = F32(a);
            a.is_infinity() || a.is_nan() || (a + a.neg()).0 == F32::ZERO.0
        }

        fn sub_works(a: f32, b: f32) -> bool {
            let sa = F32(a.to_bits());
            let sb = F32(b.to_bits());

            // (a, b) = (0.0, NaN) results in different NaNs on my machine
            sa.is_nan() || sb.is_nan() || (sa - sb).0 == (a - b).to_bits()
        }

        fn sub_is_add_neg(a: u32, b: u32) -> bool {
            let a = F32(a);
            let b = F32(b);

            a.is_nan() || b.is_nan() || a - b == a + b.neg()
        }

        fn mul_works(a: f32, b: f32) -> bool {
            let sa = F32(a.to_bits());
            let sb = F32(b.to_bits());

            (sa * sb).0 == (a * b).to_bits()
        }
    }

    #[test]
    fn test_consts() {
        assert_eq!(F32::ZERO.0, 0.0f32.to_bits());
        assert_eq!(F32::NEG_ZERO.0, (-0.0f32).to_bits());
        assert_eq!(F32::NAN.0, f32::NAN.to_bits());
        assert_eq!(F32::NEG_NAN.0, (-f32::NAN).to_bits());
        assert_eq!(F32::INFINITY.0, f32::INFINITY.to_bits());
        assert_eq!(F32::NEG_INFINITY.0, f32::NEG_INFINITY.to_bits());
        assert_eq!(F32::ONE.0, 1.0f32.to_bits());
        assert_eq!(F32::NEG_ONE.0, (-1.0f32).to_bits());
    }

    #[test]
    fn test_is_nan() {
        assert!(F32::NAN.is_nan() && F32::NEG_NAN.is_nan() && F32::from_bits(0xffc00100).is_nan());
        assert!(!F32::ONE.is_nan());

        assert!(F32::NAN.is_nan());
        assert!(F32::NEG_NAN.is_nan());
        assert!(!F32::INFINITY.is_nan());
        assert!(!F32::NEG_INFINITY.is_nan());
        assert!(!F32::ZERO.is_nan());
        assert!(!F32::NEG_ZERO.is_nan());
        assert!(!F32::ONE.is_nan());
        assert!(!F32::NEG_ONE.is_nan());

        // most significant bit not set
        assert!(F32::from_bits(0x7f800001).is_nan());
        // most significant bit set
        assert!(F32::from_bits(0x7fc00001).is_nan());
        // most significant bit and sign bit set
        assert!(F32::from_bits(0xffc00001).is_nan());
        // most significant bit not set and sign bit set
        assert!(F32::from_bits(0xff801001).is_nan());
    }

    #[test]
    fn test_is_infinity() {
        assert!(F32::INFINITY.is_infinity() && F32::NEG_INFINITY.is_infinity());
        assert!(!F32::NAN.is_infinity());
        assert!(!F32::ONE.is_infinity());
    }

    #[test]
    fn test_is_zero() {
        assert!(F32::ZERO.is_zero() && F32::NEG_ZERO.is_zero());
        assert!(!F32::NAN.is_zero());
        assert!(!F32::ONE.is_zero());
    }

    #[test]
    fn test_add() {
        // normal addition
        assert_eq!(F32::ONE + F32::ZERO, F32::ONE);
        assert_eq!(F32::ONE + F32::ONE, F32::from_bits(2f32.to_bits()));

        // NaN propagation
        assert_eq!((F32::NAN + F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NAN + F32::NEG_NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN + F32::NAN).0, F32::NEG_NAN.0);
        assert_eq!((F32::NEG_NAN + F32::NEG_NAN).0, F32::NEG_NAN.0);
        assert_eq!((F32::NAN + F32::INFINITY).0, F32::NAN.0);
        assert_eq!((F32::ONE + F32::NEG_NAN).0, F32::NEG_NAN.0);
        // all NaNs are canonical, so result should also be canonical
        assert_eq!((F32::NAN + F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN + F32::NAN).0, F32::NEG_NAN.0);

        // Infinities
        assert_eq!((F32::INFINITY + F32::NEG_INFINITY).0, F32::NAN.0);
        assert_eq!(F32::INFINITY + F32::INFINITY, F32::INFINITY);
        assert_eq!(F32::NEG_INFINITY + F32::NEG_INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::NEG_INFINITY + F32::ONE, F32::NEG_INFINITY);
        assert_eq!(F32::ONE + F32::NEG_INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::INFINITY + F32::ONE, F32::INFINITY);
        assert_eq!(F32::ONE + F32::INFINITY, F32::INFINITY);

        // Zeroes
        assert_eq!((F32::ZERO + F32::NEG_ZERO).0, F32::ZERO.0);
        assert_eq!((F32::ZERO + F32::ZERO).0, F32::ZERO.0);
        assert_eq!((F32::NEG_ZERO + F32::NEG_ZERO).0, F32::NEG_ZERO.0);
        assert_eq!(F32::ZERO + F32::INFINITY, F32::INFINITY);
        assert_eq!(F32::INFINITY + F32::ZERO, F32::INFINITY);
        assert_eq!(F32::NEG_ZERO + F32::NEG_INFINITY, F32::NEG_INFINITY);

        // TODO: rounding
    }

    #[test]
    fn test_sub() {
        // normal subtraction
        assert_eq!(F32(2.0f32.to_bits()) - F32::ONE, F32::ONE);

        // NaN propagation
        assert_eq!((F32::NAN - F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NAN - F32::NEG_NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN - F32::NAN).0, F32::NEG_NAN.0);
        assert_eq!((F32::NEG_NAN - F32::NEG_NAN).0, F32::NEG_NAN.0);
        assert_eq!((F32::NAN - F32::INFINITY).0, F32::NAN.0);
        // TODO: it's somewhat surprising that this is not NEG_NAN, but it's within spec
        assert_eq!((F32::ONE - F32::NEG_NAN).0, F32::NAN.0);
        // all NaNs are canonical, so result should also be canonical
        assert_eq!((F32::NAN - F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN - F32::NAN).0, F32::NEG_NAN.0);

        // Infinities
        assert_eq!((F32::INFINITY - F32::INFINITY).0, F32::NAN.0);
        assert_eq!((F32::NEG_INFINITY - F32::NEG_INFINITY).0, F32::NAN.0);
        assert_eq!(F32::INFINITY - F32::NEG_INFINITY, F32::INFINITY);
        assert_eq!(F32::NEG_INFINITY - F32::INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::INFINITY - F32::ONE, F32::INFINITY);
        assert_eq!(F32::NEG_INFINITY - F32::ONE, F32::NEG_INFINITY);
        assert_eq!(F32::ONE - F32::INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::ONE - F32::NEG_INFINITY, F32::INFINITY);

        // Zeroes
        assert_eq!((F32::ZERO - F32::ZERO).0, F32::ZERO.0);
        assert_eq!((F32::NEG_ZERO - F32::NEG_ZERO).0, F32::ZERO.0);
        assert_eq!((F32::ZERO - F32::NEG_ZERO).0, F32::ZERO.0);
        assert_eq!((F32::NEG_ZERO - F32::ZERO).0, F32::NEG_ZERO.0);
        assert_eq!(F32::ONE - F32::ZERO, F32::ONE);
        assert_eq!(F32::ONE - F32::NEG_ZERO, F32::ONE);
        assert_eq!(F32::ZERO - F32::ONE, F32::NEG_ONE);
        assert_eq!(F32::NEG_ZERO - F32::ONE, F32::NEG_ONE);
        assert_eq!((F32::ONE - F32::ONE).0, F32::ZERO.0);

        // TODO: rounding
    }

    #[test]
    fn test_mul() {
        // normal multiplication
        assert_eq!(
            F32(2.0f32.to_bits()) * F32(2.0f32.to_bits()),
            F32(4.0f32.to_bits())
        );

        // NaN propagation
        assert_eq!((F32::NAN * F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NAN * F32::NEG_NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN * F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NEG_NAN * F32::NEG_NAN).0, F32::NAN.0);

        // Zero and infinity
        assert_eq!((F32::INFINITY * F32::ZERO).0, F32::NAN.0);
        assert_eq!((F32::ZERO * F32::NEG_INFINITY).0, F32::NAN.0);
        // NaN and infinity
        assert_eq!((F32::NAN * F32::INFINITY).0, F32::NAN.0);
        assert_eq!((F32::INFINITY * F32::NAN).0, F32::NAN.0);
        assert_eq!((F32::NAN * F32::NEG_INFINITY).0, F32::NAN.0);
        assert_eq!((F32::NEG_INFINITY * F32::NAN).0, F32::NAN.0);
        // Infinities
        assert_eq!(F32::INFINITY * F32::INFINITY, F32::INFINITY);
        assert_eq!(F32::NEG_INFINITY * F32::NEG_INFINITY, F32::INFINITY);
        assert_eq!(F32::INFINITY * F32::NEG_INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::NEG_INFINITY * F32::INFINITY, F32::NEG_INFINITY);
        // Infinity and value with opposite sign
        assert_eq!(F32::INFINITY * F32::NEG_ONE, F32::NEG_INFINITY);
        assert_eq!(F32::NEG_INFINITY * F32::ONE, F32::NEG_INFINITY);
        assert_eq!(F32::ONE * F32::NEG_INFINITY, F32::NEG_INFINITY);
        assert_eq!(F32::NEG_ONE * F32::INFINITY, F32::NEG_INFINITY);

        // Zeroes
        assert_eq!((F32::ZERO * F32::ZERO).0, F32::ZERO.0);
        assert_eq!((F32::NEG_ZERO * F32::NEG_ZERO).0, F32::ZERO.0);
        assert_eq!((F32::ZERO * F32::NEG_ZERO).0, F32::NEG_ZERO.0);
        assert_eq!((F32::NEG_ZERO * F32::ZERO).0, F32::NEG_ZERO.0);
    }

    #[test]
    fn test_abs() {
        assert_eq!(F32::ONE.abs(), F32::ONE);
        assert_eq!(F32::NEG_ONE.abs(), F32::ONE);
        assert_eq!(F32::ZERO.abs().0, F32::ZERO.0);
        assert_eq!(F32::NEG_ZERO.abs().0, F32::ZERO.0);
        assert_eq!(F32::INFINITY.abs(), F32::INFINITY);
        assert_eq!(F32::NEG_INFINITY.abs(), F32::INFINITY);
        assert_eq!(F32::NAN.abs().0, F32::NAN.0);
        assert_eq!(F32::NEG_NAN.abs().0, F32::NAN.0);
    }

    #[test]
    fn test_neg() {
        assert_eq!(F32::ONE.neg(), F32::NEG_ONE);
        assert_eq!(F32::NEG_ONE.neg(), F32::ONE);
        assert_eq!(F32::ZERO.neg().0, F32::NEG_ZERO.0);
        assert_eq!(F32::NEG_ZERO.neg().0, F32::ZERO.0);
        assert_eq!(F32::INFINITY.neg(), F32::NEG_INFINITY);
        assert_eq!(F32::NEG_INFINITY.neg(), F32::INFINITY);
        assert_eq!(F32::NAN.neg().0, F32::NEG_NAN.0);
        assert_eq!(F32::NEG_NAN.neg().0, F32::NAN.0);
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            F32(f32::MAX.to_bits()) + F32::ONE,
            F32((f32::MAX + 1.0f32).to_bits())
        );

        assert_eq!(
            F32(0f32.to_bits()) - F32::ONE,
            F32((0f32 - 1.0f32).to_bits())
        );

        assert_eq!(
            F32(f32::MAX.to_bits()) * F32(1.5f32.to_bits()),
            F32((f32::MAX * 1.5f32).to_bits())
        );
    }
}
