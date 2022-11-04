//! Software emulated floating point operations.
//!
//! https://webassembly.github.io/spec/core/exec/numerics.html#floating-point-operations
//! # Rounding
//! round-to-nearest ties-to-even
//!
//! # NaN propagation
//! According to WebAssembly spec, the sign and payload are non-deterministic,
//! but this crate will always return the same result for the same input,
//! regardless of CPU architecture / instruction set.
//!
//! # Scope
//! This implementation only provides very basic operations for now to fill in the gaps of the other backends.

use core::ops::Neg;

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
                (self.0 & Self::EXP_MASK) == Self::EXP_MASK && (self.0 & Self::FRAC_MASK) != 0
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
    };
}

impl_float!(F32, u32, 8);
impl_float!(F64, u64, 11);

impl F32 {
    /// Returns the same value as `self`, but with as a `F64`.
    ///
    /// Example:
    /// ```
    /// use wasm_soft_floats::{F32, F64};
    /// assert_eq!(F32::from_bits(3.678f32.to_bits()).promote().to_bits(), F64::from_bits(3.678f64.to_bits()).to_bits());
    /// ```
    pub fn promote(self) -> F64 {
        // handle NaNs
        if self.is_nan() {
            return F64::NAN;
        }

        const BIT_DIFF: usize = F64::BIT_SIZE - F32::BIT_SIZE;
        const EXP_DIFF: usize = F64::EXP_BITS - F32::EXP_BITS;
        println!("{} {}", BIT_DIFF, EXP_DIFF);
        let exp_bits = self.0 & Self::EXP_MASK;
        let frac_bits = self.0 & Self::FRAC_MASK;
        let sign_bit = self.0 & Self::SIGN_MASK;
        println!("{:032b}\n{:032b}\n{:032b}", exp_bits, frac_bits, sign_bit);
        // get all the parts of the float
        let exp_bits = (self.0 & Self::EXP_MASK) as u64;
        let frac_bits = (self.0 & Self::FRAC_MASK) as u64;
        let sign_bit = (self.0 & Self::SIGN_MASK) as u64;

        println!("{:064b}\n{:064b}\n{:064b}", exp_bits, frac_bits, sign_bit);
        // shift the bits to the right place
        let exp_bits = exp_bits << (BIT_DIFF - EXP_DIFF);
        let sign_bit = sign_bit << BIT_DIFF;
        // combine the bits
        let bits = exp_bits | frac_bits | sign_bit;
        F64(bits)
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    quickcheck! {
        fn sign_works(f: f32) -> bool {
            let sf = F32(f.to_bits());

            sf.is_sign_positive() == f.is_sign_positive()
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
}
