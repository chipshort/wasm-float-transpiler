// simple operations
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_reinterpret_f_32(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_reinterpret_f_64(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_reinterpret_i_32(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_reinterpret_i_64(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_abs(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_neg(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_eq(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ne(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_copysign(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_abs(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_neg(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_eq(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ne(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_copysign(_: u64, _: u64) -> u64 {
    panic()
}

// advanced operations
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ceil(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_floor(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_trunc(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_nearest(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sqrt(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ceil(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_floor(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_trunc(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_nearest(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sqrt(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_32(_: u32) -> i32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_32(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_sf_64(_: u64) -> i32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_uf_64(_: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_32(_: u32) -> i64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_32(_: u32) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_sf_64(_: u64) -> i64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_uf_64(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_32(_: i32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_32(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_si_64(_: i64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_convert_ui_64(_: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_demote_f_64(_: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_32(_: i32) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_32(_: u32) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_si_64(_: i64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_convert_ui_64(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_promote_f_32(_: u32) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_32(_: u32) -> i32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_32(_: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_s_sat_f_64(_: u64) -> i32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_32_trunc_u_sat_f_64(_: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_32(_: u32) -> i64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_32(_: u32) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_s_sat_f_64(_: u64) -> i64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_i_64_trunc_u_sat_f_64(_: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_lt(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_gt(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_le(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_ge(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_lt(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_gt(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_le(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_ge(_: u64, _: u64) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_add(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_sub(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_mul(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_div(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_min(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_32_max(_: u32, _: u32) -> u32 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_add(_: u64, _: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_sub(_: u64, _: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_mul(_: u64, _: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_div(_: u64, _: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_min(_: u64, _: u64) -> u64 {
    panic()
}
#[no_mangle]
pub extern "C" fn __wasm_soft_float_f_64_max(_: u64, _: u64) -> u64 {
    panic()
}

#[inline(always)]
fn panic() -> ! {
    unimplemented!("floating point operations are not supported")
}
