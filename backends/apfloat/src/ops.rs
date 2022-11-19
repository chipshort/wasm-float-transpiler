use crate::bool;
use ::rustc_apfloat::ieee::{Double, Single};
use ::rustc_apfloat::{Float, FloatConvert, Round, StatusAnd};

#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ceil(v: u32) -> u32 {
    let v = Single::from_bits(v as u128);
    v.round_to_integral(Round::TowardPositive).value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_floor(v: u32) -> u32 {
    let v = Single::from_bits(v as u128);
    v.round_to_integral(Round::TowardNegative).value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_trunc(v: u32) -> u32 {
    let v = Single::from_bits(v as u128);
    v.round_to_integral(Round::TowardZero).value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_nearest(v: u32) -> u32 {
    let v = Single::from_bits(v as u128);
    v.round_to_integral(Round::NearestTiesToEven)
        .value
        .to_bits() as u32
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_sqrt(v: u32) -> u32 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ceil(v: u64) -> u64 {
    let v = Double::from_bits(v as u128);
    v.round_to_integral(Round::TowardPositive).value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_floor(v: u64) -> u64 {
    let v = Double::from_bits(v as u128);
    v.round_to_integral(Round::TowardNegative).value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_trunc(v: u64) -> u64 {
    let v = Double::from_bits(v as u128);
    v.round_to_integral(Round::TowardZero).value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_nearest(v: u64) -> u64 {
    let v = Double::from_bits(v as u128);
    v.round_to_integral(Round::NearestTiesToEven)
        .value
        .to_bits() as u64
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_sqrt(v: u64) -> u64 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(v: u32) -> i32 {
    let v = Single::from_bits(v as u128);
    v.to_i128_r(32, Round::TowardZero, &mut false).value as i32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(v: u32) -> u32 {
    let v = Single::from_bits(v as u128);
    v.to_u128_r(32, Round::TowardZero, &mut false).value as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(v: u64) -> i32 {
    let v = Double::from_bits(v as u128);
    v.to_i128_r(32, Round::TowardZero, &mut false).value as i32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(v: u64) -> u32 {
    let v = Double::from_bits(v as u128);
    v.to_u128_r(32, Round::TowardZero, &mut false).value as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(v: u32) -> i64 {
    let v = Single::from_bits(v as u128);
    v.to_i128_r(64, Round::TowardZero, &mut false).value as i64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(v: u32) -> u64 {
    let v = Single::from_bits(v as u128);
    v.to_u128_r(64, Round::TowardZero, &mut false).value as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(v: u64) -> i64 {
    let v = Double::from_bits(v as u128);
    v.to_i128_r(64, Round::TowardZero, &mut false).value as i64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(v: u64) -> u64 {
    let v = Double::from_bits(v as u128);
    v.to_u128_r(64, Round::TowardZero, &mut false).value as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(v: i32) -> u32 {
    let res = Single::from_i128(v as i128);
    res.value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(v: u32) -> u32 {
    let res = Single::from_u128(v as u128);
    res.value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(v: i64) -> u32 {
    let res = Single::from_i128(v as i128);
    res.value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(v: u64) -> u32 {
    let res = Single::from_u128(v as u128);
    res.value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(v: u64) -> u32 {
    let res: StatusAnd<Single> = Double::from_u128(v as u128).value.convert(&mut false);
    res.value.to_bits() as u32
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(v: i32) -> u64 {
    let res = Double::from_i128(v as i128);
    res.value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(v: u32) -> u64 {
    let res = Double::from_u128(v as u128);
    res.value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(v: i64) -> u64 {
    let res = Double::from_i128(v as i128);
    res.value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(v: u64) -> u64 {
    let res = Double::from_u128(v as u128);
    res.value.to_bits() as u64
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(v: u32) -> u64 {
    let res: StatusAnd<Double> = Single::from_u128(v as u128).value.convert(&mut false);
    res.value.to_bits() as u64
}
// I think we can actually just call the non-sat versions here, since `rustc_apfloat` always saturates.
// The overflow behaviour of the non-saturating versions is just not defined in the wasm spec (which is why these exist).
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(v: u32) -> i32 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_32(v: u32) -> u32 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_64(v: u64) -> i32 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_64(v: u64) -> u32 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_32(v: u32) -> i64 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_32(v: u32) -> u64 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_64(v: u64) -> i64 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_64(v: u64) -> u64 {
//     todo!()
// }
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
    bool(Double::from_bits(a as u128) > Double::from_bits(b as u128))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_le(a: u64, b: u64) -> u32 {
    bool(Double::from_bits(a as u128) <= Double::from_bits(b as u128))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ge(a: u64, b: u64) -> u32 {
    bool(Double::from_bits(a as u128) >= Double::from_bits(b as u128))
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
