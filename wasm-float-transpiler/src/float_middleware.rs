use std::sync::{Arc, RwLock};

use loupe::MemoryUsage;
use wasmer::{
    wasmparser::Operator, FunctionMiddleware, MiddlewareError, MiddlewareReaderState,
    ModuleMiddleware,
};
use wasmer_types::{FunctionIndex, ModuleInfo};

use crate::util::FunctionTyper;

macro_rules! default_fun_index {
    ($module_info:expr, $($fun:ident as $t:ty),*) => {{
        let info = $module_info;
        let index = FunIndex {
            $(
                $fun: {
                    let signature_idx = info.signatures.push(FunctionTyper::ty(crate::math::$fun as $t));
                    let fn_idx = info.functions.push(signature_idx);
                    info.imports.insert(
                        (
                            "env".to_string(),
                            concat!("__float_math_", stringify!($fun)).to_string(),
                            info.imports.len().try_into().unwrap(),
                        ),
                        wasmer_types::ImportIndex::Function(fn_idx),
                    );
                    info.num_imported_functions += 1;
                    println!("inserted {} as {} with sig {}", stringify!($fun), fn_idx.as_u32(), signature_idx.as_u32());
                    fn_idx
                },
            )*
        };

        println!("{}", info.num_imported_functions);

        index
    }};
}

#[derive(Debug, MemoryUsage)]
pub struct FloatMiddleware {
    #[loupe(skip)]
    fun_index: RwLock<Option<FunIndex>>,
}

impl FloatMiddleware {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            fun_index: RwLock::new(None),
        })
    }
}

impl ModuleMiddleware for FloatMiddleware {
    fn generate_function_middleware(
        &self,
        _local_function_index: wasmer::LocalFunctionIndex,
    ) -> Box<dyn wasmer::FunctionMiddleware> {
        Box::new(FloatFunMiddleware {
            index: _local_function_index.as_u32(),
            fun_index: self.fun_index.read().unwrap().clone().unwrap(),
        })
    }

    fn transform_module_info(&self, module_info: &mut ModuleInfo) {
        let mut lock = self.fun_index.write().unwrap();
        if lock.is_some() {
            panic!("cannot reuse float middleware for multiple modules");
        }
        // replace all F32 in signatures with I32
        module_info.signatures.iter_mut().for_each(|(_, sig)| {
            let params: Vec<_> = sig
                .params()
                .iter()
                .map(|t| {
                    if t == &wasmer_types::Type::F32 {
                        wasmer_types::Type::I32
                    } else {
                        *t
                    }
                })
                .collect();
            let results: Vec<_> = sig
                .results()
                .iter()
                .map(|t| {
                    if t == &wasmer_types::Type::F32 {
                        wasmer_types::Type::I32
                    } else {
                        *t
                    }
                })
                .collect();

            *sig = wasmer::FunctionType::new(params, results);
        });

        // save existing functions and remove them, because imports need to come first
        let existing_funs: Vec<_> = module_info.functions.values().cloned().collect();
        module_info.functions.clear();

        // add replacement functions to the module info and save for later
        let fun_index = create_index(module_info);

        // shift indexes everywhere where functions are used
        // module_info.passive_elements //FIXME: need to adjust those too. How to do that? What is that even for?
        let names: Vec<_> = module_info.function_names.keys().cloned().collect();
        for key in names {
            let name = module_info
                .function_names
                .remove(&key)
                .expect("key should exist");
            module_info.function_names.insert(
                FunctionIndex::from_u32(key.as_u32() + NUM_REPLACEMENTS),
                name,
            );
        }

        if let Some(start) = module_info.start_function {
            module_info.start_function =
                Some(FunctionIndex::from_u32(start.as_u32() + NUM_REPLACEMENTS));
        }

        // add existing functions back in
        for sig in existing_funs {
            module_info.functions.push(sig);
        }

        module_info
            .signatures
            .iter()
            .for_each(|s| println!("sig: {:?}", s));
        *lock = Some(fun_index);
    }
}

macro_rules! define_fun_middleware {
    ($($fun:ident),*) => {
        #[derive(Debug, Clone)]
        struct FunIndex {
            $(
                $fun: wasmer_types::FunctionIndex,
            )*
        }
    }
}

macro_rules! define_math {
    ($($fun:ident as $t:ty),*,) => {
        const NUM_REPLACEMENTS: u32 = [$(stringify!($fun)),*].len() as u32;

        define_fun_middleware!($($fun),*);

        fn create_index(module_info: &mut ModuleInfo) -> FunIndex {
            default_fun_index!(module_info, $($fun as $t),*)
        }

        pub(crate) fn create_imports(store: &wasmer::Store) -> wasmer::ImportObject {
            wasmer::imports! {
                "env" => {
                    $(
                        concat!("__float_math_", stringify!($fun)) => wasmer::Function::new_native(store, crate::math::$fun),
                    )*
                }
            }
        }
    };
}

define_math!(
    mul_f32 as fn(u32, u32) -> u32,
    abs_f32 as fn(u32) -> u32,
    add_f32 as fn(u32, u32) -> u32,
    div_f32 as fn(u32, u32) -> u32,
    sub_f32 as fn(u32, u32) -> u32,
    eq_f32 as fn(u32, u32) -> u32,
    ne_f32 as fn(u32, u32) -> u32,
    ceil_f32 as fn(u32) -> u32,
    floor_f32 as fn(u32) -> u32,
    lt_f32 as fn(u32, u32) -> u32,
    le_f32 as fn(u32, u32) -> u32,
    gt_f32 as fn(u32, u32) -> u32,
    ge_f32 as fn(u32, u32) -> u32,
    min_f32 as fn(u32, u32) -> u32,
    max_f32 as fn(u32, u32) -> u32,
    f32_sqrt as fn(u32) -> u32,
    trunc_f32 as fn(u32) -> u32,
    nearest_f32 as fn(u32) -> u32,
);

#[derive(Debug, Clone)]
struct FloatFunMiddleware {
    index: u32,
    /// The index of the injected replacement functions
    fun_index: FunIndex,
}

impl FunctionMiddleware for FloatFunMiddleware {
    fn feed<'a>(
        &mut self,
        operator: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        // println!("{} op: {:?}", self.index, operator);
        let call = |index: FunctionIndex| -> Operator {
            Operator::Call {
                function_index: index.as_u32(),
            }
        };
        let new_op = match operator {
            Operator::F32Load { memarg } => Operator::I32Load { memarg },
            Operator::F32Store { memarg } => Operator::I32Store { memarg },
            Operator::F32Const { value } => {
                Operator::I32Const {
                    // this is valid, since casting from u32 to i32 is a no-op
                    // see https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
                    value: value.bits() as i32,
                }
            }
            Operator::F32Abs => call(self.fun_index.abs_f32),
            Operator::F32Add => call(self.fun_index.add_f32),
            Operator::F32Sub => call(self.fun_index.sub_f32),
            Operator::F32Mul => call(self.fun_index.mul_f32),
            Operator::F32Div => call(self.fun_index.div_f32),
            Operator::F32Eq => call(self.fun_index.eq_f32),
            Operator::F32Ne => call(self.fun_index.ne_f32),
            Operator::F32Lt => call(self.fun_index.lt_f32),
            Operator::F32Gt => call(self.fun_index.gt_f32),
            Operator::F32Ge => call(self.fun_index.ge_f32),
            Operator::F32Le => call(self.fun_index.le_f32),
            Operator::F32Min => call(self.fun_index.min_f32),
            Operator::F32Max => call(self.fun_index.max_f32),
            Operator::F32Sqrt => call(self.fun_index.f32_sqrt),
            Operator::F32Ceil => call(self.fun_index.ceil_f32),
            Operator::F32Floor => call(self.fun_index.floor_f32),
            Operator::F32Trunc => call(self.fun_index.trunc_f32),
            Operator::F32Nearest => call(self.fun_index.nearest_f32),
            // Operator::F32ReinterpretI32 => Operator::Nop),
            // Operator::I32ReinterpretF32 => Operator::Nop),

            // need to shift all other function references because we inserted our imported functions at the top
            Operator::Call { function_index } => Operator::Call {
                function_index: function_index + NUM_REPLACEMENTS,
            },
            Operator::ReturnCall { function_index } => Operator::ReturnCall {
                function_index: function_index + NUM_REPLACEMENTS,
            },
            Operator::RefFunc { function_index } => Operator::RefFunc {
                function_index: function_index + NUM_REPLACEMENTS,
            },
            _ => operator,
        };
        println!("{} new op: {:?}", self.index, new_op);
        state.push_operator(new_op);

        // println!("{:#?}", state);
        Ok(())
    }
}
