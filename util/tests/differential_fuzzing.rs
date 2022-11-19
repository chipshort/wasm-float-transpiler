use quickcheck::quickcheck;
use wasm_soft_float_utils::*;

quickcheck! {
    fn abs_works(f: f32) -> bool {
        __wasm_soft_float_f_32_abs(f.to_bits()) == f.abs().to_bits()
    }

    fn neg_works(f: f32) -> bool {
        __wasm_soft_float_f_32_neg(f.to_bits()) == (-f).to_bits()
    }

    fn copysign_works(f: f32, g: f32) -> bool {
        __wasm_soft_float_f_32_copysign(f.to_bits(), g.to_bits()) == f.copysign(g).to_bits()
    }
}
