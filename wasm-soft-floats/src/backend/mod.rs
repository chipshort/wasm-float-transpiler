#[cfg(all(feature = "softfp", not(feature = "rustc_apfloat")))]
mod softfp;
#[cfg(all(feature = "softfp", not(feature = "rustc_apfloat")))]
pub use self::softfp::*;

#[cfg(all(feature = "rustc_apfloat", not(feature = "softfp")))]
mod rustc_apfloat;
#[cfg(all(feature = "rustc_apfloat", not(feature = "softfp")))]
pub use self::rustc_apfloat::*;
