use wasmer::{FunctionType, Type};

trait Typer {
    fn ty() -> Type;
}

macro_rules! impl_typer {
    ($(($t:ty, $r:ident)),*) => {
        $(
            impl Typer for $t {
                fn ty() -> wasmer::Type {
                    wasmer::Type::$r
                }
            }
        )*
    };
}

impl_typer!((i32, I32), (u32, I32), (i64, I64), (u64, I64), (bool, I32));

pub trait FunctionTyper {
    fn ty(self) -> FunctionType;
}

macro_rules! impl_function_typer {
    ($(fn($($param:ty),*) -> $ret:ty),*) => {
       $(
            impl FunctionTyper for fn($($param),*) -> $ret {
                fn ty(self) -> FunctionType {
                    FunctionType::from(([$(<$param as Typer>::ty()),*], [<$ret as Typer>::ty()]))
                }
            }
        )*
    };
}

impl_function_typer!(
    fn() -> u32,
    fn(u32) -> u32,
    fn(u32, u32) -> u32,
    fn() -> u64,
    fn(u64) -> u64,
    fn(u64, u64) -> u64
);

#[test]
fn function_typer() {
    assert_eq!(
        FunctionTyper::ty(crate::math::mul_f32 as fn(u32, u32) -> u32),
        FunctionType::from(([Type::I32, Type::I32], [Type::I32]))
    );
    assert_eq!(
        FunctionTyper::ty(crate::math::ceil_f32 as fn(u32) -> u32),
        FunctionType::from(([Type::I32], [Type::I32]))
    );
}
