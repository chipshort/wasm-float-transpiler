use std::sync::Arc;

use loupe::MemoryUsage;
use wasmer::{
    wasmparser::Operator, FunctionMiddleware, MiddlewareError, MiddlewareReaderState,
    ModuleMiddleware,
};
use wasmer_types::ModuleInfo;

#[derive(Debug, MemoryUsage)]
pub struct PrintMiddleware;

impl PrintMiddleware {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl ModuleMiddleware for PrintMiddleware {
    fn generate_function_middleware(
        &self,
        _local_function_index: wasmer::LocalFunctionIndex,
    ) -> Box<dyn wasmer::FunctionMiddleware> {
        Box::new(PrintFunction {
            index: _local_function_index.as_u32(),
        })
    }

    fn transform_module_info(&self, module_info: &mut ModuleInfo) {}
}

#[derive(Debug, Clone)]
struct PrintFunction {
    index: u32,
}

impl FunctionMiddleware for PrintFunction {
    fn feed<'a>(
        &mut self,
        operator: Operator<'a>,
        state: &mut MiddlewareReaderState<'a>,
    ) -> Result<(), MiddlewareError> {
        println!("{} op: {:?}", self.index, operator);
        state.push_operator(operator);
        Ok(())
    }
}
