use std::cmp::Ordering;

use simple_soft_float::{RoundingMode, F32};

pub fn mul_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    a.mul(&b, None, None).into_bits()
}

pub fn abs_f32(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.abs().into_bits()
}

pub fn add_f32(a: u32, b: u32) -> u32 {
    println!("a: {:?}, b: {:?}", a, b);

    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    println!("a: {:?}, b: {:?}", a, b);

    a.add(&b, None, None).into_bits()
}

pub fn div_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    a.div(&b, None, None).into_bits()
}

pub fn sub_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    a.sub(&b, None, None).into_bits()
}

pub fn eq_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(compare(a, b, Ordering::Equal))
}

pub fn ne_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(!compare(a, b, Ordering::Equal))
}

pub fn ceil_f32(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.round_to_integral(false, Some(RoundingMode::TowardPositive), None)
        .into_bits()
}

pub fn floor_f32(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.round_to_integral(false, Some(RoundingMode::TowardNegative), None)
        .into_bits()
}

pub fn lt_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(compare(a, b, Ordering::Less))
}

pub fn gt_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(compare(a, b, Ordering::Greater))
}

// TODO: these could be optimized
pub fn le_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(compare(a, b, Ordering::Less) || compare(a, b, Ordering::Equal))
}

pub fn ge_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    bool(compare(a, b, Ordering::Greater) || compare(a, b, Ordering::Equal))
}

pub fn min_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    // special cases, TODO: not sure if needed
    if a.is_nan() || b.is_nan() {
        return F32::quiet_nan().into_bits();
    } else if a.is_negative_zero() && b.is_positive_zero() {
        return a.into_bits();
    } else if a.is_positive_zero() && b.is_negative_zero() {
        return b.into_bits();
    }
    if compare(a, b, Ordering::Less) {
        a.into_bits()
    } else {
        b.into_bits()
    }
}

pub fn max_f32(a: u32, b: u32) -> u32 {
    let a = F32::from_bits(a);
    let b = F32::from_bits(b);

    // special cases, TODO: not sure if needed
    if a.is_nan() || b.is_nan() {
        return F32::quiet_nan().into_bits();
    } else if a.is_negative_zero() && b.is_positive_zero() {
        return b.into_bits();
    } else if a.is_positive_zero() && b.is_negative_zero() {
        return a.into_bits();
    }
    if compare(a, b, Ordering::Greater) {
        a.into_bits()
    } else {
        b.into_bits()
    }
}

pub fn f32_sqrt(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.sqrt(None, None).into_bits()
}

pub fn trunc_f32(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.round_to_integral(false, Some(RoundingMode::TowardZero), None)
        .into_bits()
}

pub fn nearest_f32(v: u32) -> u32 {
    let v = F32::from_bits(v);
    v.round_to_integral(false, Some(RoundingMode::TiesToEven), None)
        .into_bits()
}

fn compare(a: F32, b: F32, ordering: Ordering) -> bool {
    // TODO: check if this is correct, using unwrap_or_default because of NaN values
    a.compare_quiet(&b, None)
        .map(|o| o == ordering)
        .unwrap_or_default()
}

fn bool(v: bool) -> u32 {
    if v {
        1
    } else {
        0
    }
}
