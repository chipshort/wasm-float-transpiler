use softfloat_c::*;
use wasm_soft_float_utils::bool;

// const SOFTFLOAT_ROUND_ODD: u8 = 6;
// const SOFTFLOAT_ROUND_NEAR_MAX_MAG: u8 = 4;
const SOFTFLOAT_ROUND_MAX: u8 = 3;
const SOFTFLOAT_ROUND_MIN: u8 = 2;
const SOFTFLOAT_ROUND_MIN_MAG: u8 = 1;
const SOFTFLOAT_ROUND_NEAR_EVEN: u8 = 0;

#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ceil(v: u32) -> u32 {
    unsafe { f32_roundToInt(float32_t::from_bits(v), SOFTFLOAT_ROUND_MAX, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_floor(v: u32) -> u32 {
    unsafe { f32_roundToInt(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_trunc(v: u32) -> u32 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f32_roundToInt(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false).to_bits() }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_nearest(v: u32) -> u32 {
    unsafe { f32_roundToInt(float32_t::from_bits(v), SOFTFLOAT_ROUND_NEAR_EVEN, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sqrt(v: u32) -> u32 {
    unsafe { f32_sqrt(float32_t::from_bits(v)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ceil(v: u64) -> u64 {
    unsafe { f64_roundToInt(float64_t::from_bits(v), SOFTFLOAT_ROUND_MAX, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_floor(v: u64) -> u64 {
    unsafe { f64_roundToInt(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_trunc(v: u64) -> u64 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f64_roundToInt(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false).to_bits() }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_nearest(v: u64) -> u64 {
    unsafe { f64_roundToInt(float64_t::from_bits(v), SOFTFLOAT_ROUND_NEAR_EVEN, false) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sqrt(v: u64) -> u64 {
    unsafe { f64_sqrt(float64_t::from_bits(v)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(v: u32) -> i32 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f32_to_i32(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) as i32 }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(v: u32) -> u32 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f32_to_ui32(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) as u32 }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(v: u64) -> i32 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f64_to_i32(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) as i32 }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(v: u64) -> u32 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f64_to_ui32(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) as u32 }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(v: u32) -> i64 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f32_to_i64(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(v: u32) -> u64 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f32_to_ui64(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(v: u64) -> i64 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f64_to_i64(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(v: u64) -> u64 {
    // TODO: softfloat_round_minMag or softfloat_round_near_maxMag?
    unsafe { f64_to_ui64(float64_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, false) }
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(v: i32) -> u32 {
    unsafe { i32_to_f32(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(v: u32) -> u32 {
    unsafe { ui32_to_f32(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(v: i64) -> u32 {
    unsafe { i64_to_f32(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(v: u64) -> u32 {
    unsafe { ui64_to_f32(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(v: u64) -> u32 {
    unsafe { f64_to_f32(float64_t::from_bits(v)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(v: i32) -> u64 {
    unsafe { i32_to_f64(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(v: u32) -> u64 {
    unsafe { ui32_to_f64(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(v: i64) -> u64 {
    unsafe { i64_to_f64(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(v: u64) -> u64 {
    unsafe { ui64_to_f64(v) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(v: u32) -> u64 {
    unsafe { f32_to_f64(float32_t::from_bits(v)) }.to_bits()
}
// TODO: implement those
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(v: u32) -> i32 {
//     unsafe { f32_to_i32(float32_t::from_bits(v), SOFTFLOAT_ROUND_MIN_MAG, true) as i32 }
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
    bool(unsafe { f32_lt(float32_t::from_bits(a), float32_t::from_bits(b)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_gt(a: u32, b: u32) -> u32 {
    bool(unsafe { f32_lt(float32_t::from_bits(b), float32_t::from_bits(a)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_le(a: u32, b: u32) -> u32 {
    bool(unsafe { f32_le(float32_t::from_bits(a), float32_t::from_bits(b)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ge(a: u32, b: u32) -> u32 {
    bool(unsafe { f32_le(float32_t::from_bits(b), float32_t::from_bits(a)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_lt(a: u64, b: u64) -> u32 {
    bool(unsafe { f64_lt(float64_t::from_bits(a), float64_t::from_bits(b)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_gt(a: u64, b: u64) -> u32 {
    bool(unsafe { f64_lt(float64_t::from_bits(b), float64_t::from_bits(a)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_le(a: u64, b: u64) -> u32 {
    bool(unsafe { f64_le(float64_t::from_bits(a), float64_t::from_bits(b)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ge(a: u64, b: u64) -> u32 {
    bool(unsafe { f64_le(float64_t::from_bits(b), float64_t::from_bits(a)) })
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_add(a: u32, b: u32) -> u32 {
    unsafe { f32_add(float32_t::from_bits(a), float32_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sub(a: u32, b: u32) -> u32 {
    unsafe { f32_sub(float32_t::from_bits(a), float32_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_mul(a: u32, b: u32) -> u32 {
    unsafe { f32_mul(float32_t::from_bits(a), float32_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_div(a: u32, b: u32) -> u32 {
    unsafe { f32_div(float32_t::from_bits(a), float32_t::from_bits(b)) }.to_bits()
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_min(a: u32, b: u32) -> u32 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_max(a: u32, b: u32) -> u32 {
//     todo!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_add(a: u64, b: u64) -> u64 {
    unsafe { f64_add(float64_t::from_bits(a), float64_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sub(a: u64, b: u64) -> u64 {
    unsafe { f64_sub(float64_t::from_bits(a), float64_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_mul(a: u64, b: u64) -> u64 {
    unsafe { f64_mul(float64_t::from_bits(a), float64_t::from_bits(b)) }.to_bits()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_div(a: u64, b: u64) -> u64 {
    unsafe { f64_div(float64_t::from_bits(a), float64_t::from_bits(b)) }.to_bits()
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_min(a: u64, b: u64) -> u64 {
//     todo!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_max(a: u64, b: u64) -> u64 {
//     todo!()
// }
