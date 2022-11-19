use softfp::{F32, F64};
use wasm_soft_float_utils::bool;

// special functions needed for the softfp crate
#[no_mangle]
fn softfp_get_rounding_mode() -> softfp::RoundingMode {
    softfp::RoundingMode::TiesToEven
}
#[no_mangle]
fn softfp_set_exception_flags(_flags: softfp::ExceptionFlags) {}

// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_ceil(v: u32) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_floor(v: u32) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_trunc(v: u32) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_32_nearest(v: u32) -> u32 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sqrt(v: u32) -> u32 {
    F32::new(v).square_root().0
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_ceil(v: u64) -> u64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_floor(v: u64) -> u64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_trunc(v: u64) -> u64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_f_64_nearest(v: u64) -> u64 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sqrt(v: u64) -> u64 {
    F64::new(v).square_root().0
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(v: u32) -> i32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(v: u32) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(v: u64) -> i32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(v: u64) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(v: u32) -> i64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(v: u32) -> u64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(v: u64) -> i64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(v: u64) -> u64 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(v: i32) -> u32 {
    F32::convert_from_sint(v as u32).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(v: u32) -> u32 {
    F32::convert_from_uint(v).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(v: i64) -> u32 {
    F32::convert_from_sint(v as u64).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(v: u64) -> u32 {
    F32::convert_from_uint(v).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(v: u64) -> u32 {
    let v: F32 = F64::new(v).convert_format();
    v.0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(v: i32) -> u64 {
    F64::convert_from_sint(v as u32).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(v: u32) -> u64 {
    F64::convert_from_uint(v).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(v: i64) -> u64 {
    F64::convert_from_sint(v as u64).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(v: u64) -> u64 {
    F64::convert_from_uint(v).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(v: u32) -> u64 {
    let v: F64 = F32::new(v).convert_format();
    v.0
}
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(v: u32) -> i32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_32(v: u32) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_64(v: u64) -> i32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_64(v: u64) -> u32 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_32(v: u32) -> i64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_32(v: u32) -> u64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_64(v: u64) -> i64 {
//     unimplemented!()
// }
// #[no_mangle]
// pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_64(v: u64) -> u64 {
//     unimplemented!()
// }
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_lt(a: u32, b: u32) -> u32 {
    bool(F32::new(a) < F32::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_gt(a: u32, b: u32) -> u32 {
    bool(F32::new(a) > F32::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_le(a: u32, b: u32) -> u32 {
    bool(F32::new(a) <= F32::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ge(a: u32, b: u32) -> u32 {
    bool(F32::new(a) >= F32::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_lt(a: u64, b: u64) -> u32 {
    bool(F64::new(a) < F64::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_gt(a: u64, b: u64) -> u32 {
    bool(F64::new(a) > F64::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_le(a: u64, b: u64) -> u32 {
    bool(F64::new(a) <= F64::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ge(a: u64, b: u64) -> u32 {
    bool(F64::new(a) >= F64::new(b))
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_add(a: u32, b: u32) -> u32 {
    (F32::new(a) + F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sub(a: u32, b: u32) -> u32 {
    (F32::new(a) - F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_mul(a: u32, b: u32) -> u32 {
    (F32::new(a) * F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_div(a: u32, b: u32) -> u32 {
    (F32::new(a) / F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_min(a: u32, b: u32) -> u32 {
    F32::min(F32::new(a), F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_max(a: u32, b: u32) -> u32 {
    F32::max(F32::new(a), F32::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_add(a: u64, b: u64) -> u64 {
    (F64::new(a) + F64::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sub(a: u64, b: u64) -> u64 {
    (F64::new(a) - F64::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_mul(a: u64, b: u64) -> u64 {
    (F64::new(a) * F64::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_div(a: u64, b: u64) -> u64 {
    (F64::new(a) / F64::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_min(a: u64, b: u64) -> u64 {
    F64::min(F64::new(a), F64::new(b)).0
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_max(a: u64, b: u64) -> u64 {
    F64::max(F64::new(a), F64::new(b)).0
}
