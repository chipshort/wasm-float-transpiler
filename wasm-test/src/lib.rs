// extern "C" {
//     fn __float_math_mul_f32(a: u32, b: u32) -> u32;
//     fn __float_math_abs_f32(a: u32) -> u32;
//     fn __float_math_add_f32(a: u32, b: u32) -> u32;
//     fn __float_math_div_f32(a: u32, b: u32) -> u32;
//     fn __float_math_sub_f32(a: u32, b: u32) -> u32;
//     fn __float_math_eq_f32(a: u32, b: u32) -> u32;
//     fn __float_math_ne_f32(a: u32, b: u32) -> u32;
//     fn __float_math_ceil_f32(a: u32) -> u32;
//     fn __float_math_floor_f32(a: u32) -> u32;
//     fn __float_math_lt_f32(a: u32, b: u32) -> u32;
//     fn __float_math_le_f32(a: u32, b: u32) -> u32;
//     fn __float_math_gt_f32(a: u32, b: u32) -> u32;
//     fn __float_math_ge_f32(a: u32, b: u32) -> u32;
//     fn __float_math_min_f32(a: u32, b: u32) -> u32;
//     fn __float_math_max_f32(a: u32, b: u32) -> u32;
//     fn __float_math_f32_sqrt(a: u32) -> u32;
//     fn __float_math_trunc_f32(a: u32) -> u32;
//     fn __float_math_nearest_f32(a: u32) -> u32;
// }

// #[no_mangle]
// pub extern "C" fn test_f32(a: f32, b: f32) -> f32 {
//     let v = 2.0 * b;
//     let v = v + a;
//     let v = v / 2.0;
//     let v = v - a;
//     let v = v.abs();
//     let v = v.max(b);

//     if a <= b || a + 1.0 < b || v >= b || v >= a {
//         return 2.0;
//     }

//     v.min(4.0)
// }

#[no_mangle]
#[inline(never)]
pub extern "C" fn test_f32(a: f32, b: f32) -> f32 {
    // unsafe { test_fun(23) }

    // test2(b) + a
    a + b
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn test2(a: f32) -> f32 {
    // unsafe {
    //     let a = 2;
    //     __float_math_mul_f32(a, 5);
    //     __float_math_abs_f32(a);
    //     __float_math_add_f32(a, 5);
    //     __float_math_div_f32(a, 5);
    //     __float_math_sub_f32(a, 5);
    //     __float_math_eq_f32(a, 5);
    //     __float_math_ne_f32(a, 5);
    //     __float_math_ceil_f32(a);
    //     __float_math_floor_f32(a);
    //     __float_math_lt_f32(a, 5);
    //     __float_math_le_f32(a, 5);
    //     __float_math_gt_f32(a, 5);
    //     __float_math_ge_f32(a, 5);
    //     __float_math_min_f32(a, 5);
    //     __float_math_max_f32(a, 5);
    //     __float_math_f32_sqrt(a);
    //     __float_math_trunc_f32(a);
    //     __float_math_nearest_f32(a);
    // }
    5.0 * a
}
