#![no_std]
mod float;

use core::ops::Neg;

pub use float::F32;

// #[no_mangle]
// pub extern "C" fn f32_add(a: u32, b: u32) -> u32 {
//     let a = F32::from_bits(a);
//     let b = F32::from_bits(b);

//     (a + b).to_bits()
// }

// #[no_mangle]
// pub extern "C" fn f32_sub(a: u32, b: u32) -> u32 {
//     let a = F32::from_bits(a);
//     let b = F32::from_bits(b);

//     (a - b).to_bits()
// }

// #[no_mangle]
// pub extern "C" fn f32_mul(a: u32, b: u32) -> u32 {
//     let a = F32::from_bits(a);
//     let b = F32::from_bits(b);

//     (a * b).to_bits()
// }

// #[no_mangle]
// pub extern "C" fn f32_div(a: u32, b: u32) -> u32 {
//     let a = F32::from_bits(a);
//     let b = F32::from_bits(b);

//     (a / b).to_bits()
// }

// TODO: f32_min
// TODO: f32_max

#[no_mangle]
pub extern "C" fn f32_copysign(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    a.copy_sign(b).to_bits()
}

#[no_mangle]
pub extern "C" fn f32_abs(v: u32) -> u32 {
    F32::from_bits(v).abs().to_bits()
}

#[no_mangle]
pub extern "C" fn f32_neg(v: u32) -> u32 {
    F32::from_bits(v).neg().to_bits()
}

// f32_sqrt
// f32_ceil
// f32_floor
// f32_trunc
// f32_nearest

#[no_mangle]
pub extern "C" fn f32_eq(a: u32, b: u32) -> u32 {
    bool(F32::from_bits(a) == F32::from_bits(b))
}

#[no_mangle]
pub extern "C" fn f32_ne(a: u32, b: u32) -> u32 {
    bool(F32::from_bits(a) != F32::from_bits(b))
}

// f32_lt
// f32_gt
// f32_le
// f32_ge
// f32_pmin
// f32_pmax

#[no_mangle]
pub extern "C" fn f32_copy_sign(a: u32, b: u32) -> u32 {
    F32::from_bits(a).copy_sign(F32::from_bits(b)).to_bits()
}

// i32_trunc_f32_u, etc.
// f32_demote_f64
// f32_convert_i32_s, etc.

// reinterpret instructions are handled as noop in the transpiler
// load and store instructions are converted to integer loads / stores in the transpiler

#[inline(always)]
fn bool(v: bool) -> u32 {
    if v {
        1
    } else {
        0
    }
}
