use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use const_format::{concatcp, map_ascii_case, Case};
use rayon::prelude::*;
use walrus::{
    ir::{
        dfs_pre_order_mut, Binop, Call, Const, Instr, Load, LoadKind, RefNull, Select, Store,
        StoreKind, Unop, Value, VisitorMut,
    },
    ExportItem, FunctionId, Module, ValType,
};

use crate::SOFTFLOAT_PREFIX;

pub fn replace_float_operations(module: &mut Module) -> Result<()> {
    // map from soft float function name to function id
    let softfloat_funcs: HashMap<_, _> = module
        .exports
        .iter()
        .filter(|e| e.name.starts_with(SOFTFLOAT_PREFIX))
        .filter_map(|e| match e.item {
            ExportItem::Function(id) => Some((e.name.as_str(), id)),
            _ => None,
        })
        .collect();
    let softfloat_funcs = Arc::new(softfloat_funcs);

    // replace all float operations with calls to soft float functions
    module.funcs.par_iter_local_mut().for_each(|(_, func)| {
        let mut visitor = FloatReplacer {
            replacement_funcs: softfloat_funcs.clone(),
        };
        dfs_pre_order_mut(&mut visitor, func, func.entry_block());
    });

    // replace function types
    for (_, func) in module.funcs.iter_local() {
        let ty = module.types.get_mut(func.ty());
        for p in ty.params_mut() {
            change_type(p);
        }
        for r in ty.results_mut() {
            change_type(r);
        }
    }

    for l in module.locals.iter_mut() {
        change_type(&mut l.ty);
    }

    // remove soft float exports
    for export in module
        .exports
        .iter()
        .filter(|e| e.name.starts_with(SOFTFLOAT_PREFIX))
        .map(|e| e.id())
        .collect::<Vec<_>>()
    {
        module.exports.delete(export);
    }

    Ok(())
}

fn change_type(t: &mut ValType) {
    if *t == ValType::F32 {
        *t = ValType::I32;
    } else if *t == ValType::F64 {
        *t = ValType::I64;
    }
}

struct FloatReplacer<'a> {
    replacement_funcs: Arc<HashMap<&'a str, FunctionId>>,
}

impl VisitorMut for FloatReplacer<'_> {
    fn visit_instr_mut(&mut self, instr: &mut Instr, _instr_loc: &mut walrus::InstrLocId) {
        macro_rules! call_fn {
            ($name: expr) => {{
                const NAME: &str = $name;
                *instr = Instr::Call(Call {
                    func: *self
                        .replacement_funcs
                        .get(concatcp!(crate::SOFTFLOAT_PREFIX, NAME))
                        .expect(concatcp!("no function ", NAME, " found")), // TODO: error message
                })
            }};
        }
        macro_rules! match_unops {
            ($op: expr, $($name: tt),*) => {
                match $op {
                    $(
                        walrus::ir::UnaryOp::$name => call_fn!(map_ascii_case!(Case::Snake, stringify!($name))),
                    )*
                    _ => {}
                }
            };
        }
        macro_rules! match_binops {
            ($op: expr, $($name: tt),*) => {
                match $op {
                    $(
                        walrus::ir::BinaryOp::$name => call_fn!(map_ascii_case!(Case::Snake, stringify!($name))),
                    )*
                    _ => {}
                }
            };
        }
        match instr {
            Instr::Const(Const { value }) => match value {
                Value::F32(v) => *value = Value::I32(v.to_bits() as i32),
                Value::F64(v) => *value = Value::I64(v.to_bits() as i64),
                _ => {}
            },
            Instr::Load(Load { kind, .. }) => match kind {
                LoadKind::F32 => *kind = LoadKind::I32 { atomic: false },
                LoadKind::F64 => *kind = LoadKind::I64 { atomic: false },
                _ => {}
            },
            Instr::Store(Store { kind, .. }) => match kind {
                StoreKind::F32 => *kind = StoreKind::I32 { atomic: false },
                StoreKind::F64 => *kind = StoreKind::I64 { atomic: false },
                _ => {}
            },
            Instr::Select(Select { ty: Some(ty) }) => change_type(ty),
            Instr::RefNull(RefNull { ty }) => change_type(ty),
            Instr::Unop(Unop { op }) => {
                match_unops!(
                    op,
                    F32Abs,
                    F32Neg,
                    F32Ceil,
                    F32Floor,
                    F32Trunc,
                    F32Nearest,
                    F32Sqrt,
                    F64Abs,
                    F64Neg,
                    F64Ceil,
                    F64Floor,
                    F64Trunc,
                    F64Nearest,
                    F64Sqrt,
                    I32TruncSF32,
                    I32TruncUF32,
                    I32TruncSF64,
                    I32TruncUF64,
                    I64TruncSF32,
                    I64TruncUF32,
                    I64TruncSF64,
                    I64TruncUF64,
                    F32ConvertSI32,
                    F32ConvertUI32,
                    F32ConvertSI64,
                    F32ConvertUI64,
                    F32DemoteF64,
                    F64ConvertSI32,
                    F64ConvertUI32,
                    F64ConvertSI64,
                    F64ConvertUI64,
                    F64PromoteF32,
                    I32ReinterpretF32,
                    I64ReinterpretF64,
                    F32ReinterpretI32,
                    F64ReinterpretI64
                );
            }
            Instr::Binop(Binop { op }) => {
                match_binops!(
                    op,
                    F32Eq,
                    F32Ne,
                    F32Lt,
                    F32Gt,
                    F32Le,
                    F32Ge,
                    F64Eq,
                    F64Ne,
                    F64Lt,
                    F64Gt,
                    F64Le,
                    F64Ge,
                    F32Add,
                    F32Sub,
                    F32Mul,
                    F32Div,
                    F32Min,
                    F32Max,
                    F32Copysign,
                    F64Add,
                    F64Sub,
                    F64Mul,
                    F64Div,
                    F64Min,
                    F64Max,
                    F64Copysign
                );
            }
            _ => {}
        }
    }
}
