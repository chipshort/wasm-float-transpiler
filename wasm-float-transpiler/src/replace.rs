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
        macro_rules! call_op {
            ($name: path) => {
                call_fn!(map_ascii_case!(Case::Snake, stringify!($name)))
            };
        }
        macro_rules! match_unop {
            ($name: tt) => {
                walrus::ir::UnaryOp::$name
            };
            ($name: tt { .. }) => {
                walrus::ir::UnaryOp::$name { .. }
            };
        }
        macro_rules! match_binop {
            ($name: tt) => {
                walrus::ir::BinaryOp::$name
            };
            ($name: tt { .. }) => {
                walrus::ir::BinaryOp::$name { .. }
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
                match op {
                    match_unop!(F32Abs) => call_op!(F32Abs),
                    match_unop!(F32Neg) => call_op!(F32Neg),
                    match_unop!(F32Ceil) => call_op!(F32Ceil),
                    match_unop!(F32Floor) => call_op!(F32Floor),
                    match_unop!(F32Trunc) => call_op!(F32Trunc),
                    match_unop!(F32Nearest) => call_op!(F32Nearest),
                    match_unop!(F32Sqrt) => call_op!(F32Sqrt),
                    match_unop!(F64Abs) => call_op!(F64Abs),
                    match_unop!(F64Neg) => call_op!(F64Neg),
                    match_unop!(F64Ceil) => call_op!(F64Ceil),
                    match_unop!(F64Floor) => call_op!(F64Floor),
                    match_unop!(F64Trunc) => call_op!(F64Trunc),
                    match_unop!(F64Nearest) => call_op!(F64Nearest),
                    match_unop!(F64Sqrt) => call_op!(F64Sqrt),
                    match_unop!(I32TruncSF32) => call_op!(I32TruncSF32),
                    match_unop!(I32TruncUF32) => call_op!(I32TruncUF32),
                    match_unop!(I32TruncSF64) => call_op!(I32TruncSF64),
                    match_unop!(I32TruncUF64) => call_op!(I32TruncUF64),
                    match_unop!(I64TruncSF32) => call_op!(I64TruncSF32),
                    match_unop!(I64TruncUF32) => call_op!(I64TruncUF32),
                    match_unop!(I64TruncSF64) => call_op!(I64TruncSF64),
                    match_unop!(I64TruncUF64) => call_op!(I64TruncUF64),
                    match_unop!(F32ConvertSI32) => call_op!(F32ConvertSI32),
                    match_unop!(F32ConvertUI32) => call_op!(F32ConvertUI32),
                    match_unop!(F32ConvertSI64) => call_op!(F32ConvertSI64),
                    match_unop!(F32ConvertUI64) => call_op!(F32ConvertUI64),
                    match_unop!(F32DemoteF64) => call_op!(F32DemoteF64),
                    match_unop!(F64ConvertSI32) => call_op!(F64ConvertSI32),
                    match_unop!(F64ConvertUI32) => call_op!(F64ConvertUI32),
                    match_unop!(F64ConvertSI64) => call_op!(F64ConvertSI64),
                    match_unop!(F64ConvertUI64) => call_op!(F64ConvertUI64),
                    match_unop!(F64PromoteF32) => call_op!(F64PromoteF32),
                    match_unop!(I32ReinterpretF32) => call_op!(I32ReinterpretF32),
                    match_unop!(I64ReinterpretF64) => call_op!(I64ReinterpretF64),
                    match_unop!(F32ReinterpretI32) => call_op!(F32ReinterpretI32),
                    match_unop!(F64ReinterpretI64) => call_op!(F64ReinterpretI64),
                    match_unop!(I32TruncSSatF32) => call_op!(I32TruncSSatF32),
                    match_unop!(I32TruncUSatF32) => call_op!(I32TruncUSatF32),
                    match_unop!(I32TruncSSatF64) => call_op!(I32TruncSSatF64),
                    match_unop!(I32TruncUSatF64) => call_op!(I32TruncUSatF64),
                    match_unop!(I64TruncSSatF32) => call_op!(I64TruncSSatF32),
                    match_unop!(I64TruncUSatF32) => call_op!(I64TruncUSatF32),
                    match_unop!(I64TruncSSatF64) => call_op!(I64TruncSSatF64),
                    match_unop!(I64TruncUSatF64) => call_op!(I64TruncUSatF64),
                    // the following instructions are currently not supported in the softfloat library,
                    // but are checked here in order to avoid accidentally keeing them in the wasm without warning.
                    // This is necessary, since we change all types to i32 / i64).
                    match_unop!(F32x4Splat) => call_op!(F32x4Splat),
                    match_unop!(F32x4ExtractLane { .. }) => call_op!(F32x4ExtractLane),
                    match_unop!(F64x2Splat) => call_op!(F64x2Splat),
                    match_unop!(F64x2ExtractLane { .. }) => call_op!(F64x2ExtractLane),
                    match_unop!(F32x4Ceil) => call_op!(F32x4Ceil),
                    match_unop!(F32x4Floor) => call_op!(F32x4Floor),
                    match_unop!(F32x4Trunc) => call_op!(F32x4Trunc),
                    match_unop!(F32x4Nearest) => call_op!(F32x4Nearest),
                    match_unop!(F64x2Ceil) => call_op!(F64x2Ceil),
                    match_unop!(F64x2Floor) => call_op!(F64x2Floor),
                    match_unop!(F64x2Trunc) => call_op!(F64x2Trunc),
                    match_unop!(F64x2Nearest) => call_op!(F64x2Nearest),
                    match_unop!(F32x4Abs) => call_op!(F32x4Abs),
                    match_unop!(F32x4Neg) => call_op!(F32x4Neg),
                    match_unop!(F32x4Sqrt) => call_op!(F32x4Sqrt),
                    match_unop!(F64x2Abs) => call_op!(F64x2Abs),
                    match_unop!(F64x2Neg) => call_op!(F64x2Neg),
                    match_unop!(F64x2Sqrt) => call_op!(F64x2Sqrt),
                    match_unop!(I32x4TruncSatF32x4S) => call_op!(I32x4TruncSatF32x4S),
                    match_unop!(I32x4TruncSatF32x4U) => call_op!(I32x4TruncSatF32x4U),
                    match_unop!(F32x4ConvertI32x4S) => call_op!(F32x4ConvertI32x4S),
                    match_unop!(F32x4ConvertI32x4U) => call_op!(F32x4ConvertI32x4U),
                    _ => {}
                };
            }
            Instr::Binop(Binop { op }) => match op {
                match_binop!(F32Eq) => call_op!(F32Eq),
                match_binop!(F32Ne) => call_op!(F32Ne),
                match_binop!(F32Lt) => call_op!(F32Lt),
                match_binop!(F32Gt) => call_op!(F32Gt),
                match_binop!(F32Le) => call_op!(F32Le),
                match_binop!(F32Ge) => call_op!(F32Ge),
                match_binop!(F64Eq) => call_op!(F64Eq),
                match_binop!(F64Ne) => call_op!(F64Ne),
                match_binop!(F64Lt) => call_op!(F64Lt),
                match_binop!(F64Gt) => call_op!(F64Gt),
                match_binop!(F64Le) => call_op!(F64Le),
                match_binop!(F64Ge) => call_op!(F64Ge),
                match_binop!(F32Add) => call_op!(F32Add),
                match_binop!(F32Sub) => call_op!(F32Sub),
                match_binop!(F32Mul) => call_op!(F32Mul),
                match_binop!(F32Div) => call_op!(F32Div),
                match_binop!(F32Min) => call_op!(F32Min),
                match_binop!(F32Max) => call_op!(F32Max),
                match_binop!(F32Copysign) => call_op!(F32Copysign),
                match_binop!(F64Add) => call_op!(F64Add),
                match_binop!(F64Sub) => call_op!(F64Sub),
                match_binop!(F64Mul) => call_op!(F64Mul),
                match_binop!(F64Div) => call_op!(F64Div),
                match_binop!(F64Min) => call_op!(F64Min),
                match_binop!(F64Max) => call_op!(F64Max),
                match_binop!(F64Copysign) => call_op!(F64Copysign),
                // the following instructions are currently not supported in the softfloat library,
                // see above for more info
                match_binop!(F32x4ReplaceLane { .. }) => call_op!(F32x4ReplaceLane),
                match_binop!(F64x2ReplaceLane { .. }) => call_op!(F64x2ReplaceLane),
                match_binop!(F32x4Eq) => call_op!(F32x4Eq),
                match_binop!(F32x4Ne) => call_op!(F32x4Ne),
                match_binop!(F32x4Lt) => call_op!(F32x4Lt),
                match_binop!(F32x4Gt) => call_op!(F32x4Gt),
                match_binop!(F32x4Le) => call_op!(F32x4Le),
                match_binop!(F32x4Ge) => call_op!(F32x4Ge),
                match_binop!(F64x2Eq) => call_op!(F64x2Eq),
                match_binop!(F64x2Ne) => call_op!(F64x2Ne),
                match_binop!(F64x2Lt) => call_op!(F64x2Lt),
                match_binop!(F64x2Gt) => call_op!(F64x2Gt),
                match_binop!(F64x2Le) => call_op!(F64x2Le),
                match_binop!(F64x2Ge) => call_op!(F64x2Ge),
                match_binop!(F32x4Add) => call_op!(F32x4Add),
                match_binop!(F32x4Sub) => call_op!(F32x4Sub),
                match_binop!(F32x4Mul) => call_op!(F32x4Mul),
                match_binop!(F32x4Div) => call_op!(F32x4Div),
                match_binop!(F32x4Min) => call_op!(F32x4Min),
                match_binop!(F32x4Max) => call_op!(F32x4Max),
                match_binop!(F32x4PMin) => call_op!(F32x4PMin),
                match_binop!(F32x4PMax) => call_op!(F32x4PMax),
                match_binop!(F64x2Add) => call_op!(F64x2Add),
                match_binop!(F64x2Sub) => call_op!(F64x2Sub),
                match_binop!(F64x2Mul) => call_op!(F64x2Mul),
                match_binop!(F64x2Div) => call_op!(F64x2Div),
                match_binop!(F64x2Min) => call_op!(F64x2Min),
                match_binop!(F64x2Max) => call_op!(F64x2Max),
                match_binop!(F64x2PMin) => call_op!(F64x2PMin),
                match_binop!(F64x2PMax) => call_op!(F64x2PMax),
                // these are not implemented yet in the `wasmparser` version backing `walrus`
                // match_binop!(F32x4RelaxedMin) => call_op!(F32x4RelaxedMin),
                // match_binop!(F32x4RelaxedMax) => call_op!(F32x4RelaxedMax),
                // match_binop!(F64x2RelaxedMin) => call_op!(F64x2RelaxedMin),
                // match_binop!(F64x2RelaxedMax) => call_op!(F64x2RelaxedMax),
                // match_binop!(F32x4Fma) => call_op!(F32x4Fma),
                // match_binop!(F32x4Fms) => call_op!(F32x4Fms),
                // match_binop!(F64x2Fma) => call_op!(F64x2Fma),
                // match_binop!(F64x2Fm) => call_op!(F64x2Fm),
                _ => {}
            },
            _ => {}
        }
    }
}
