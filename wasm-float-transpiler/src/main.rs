mod float_middleware;
mod gatekeeper;
mod math;
mod print_middleware;
mod util;
#[cfg(test)]
mod wast;

use wasmer::CompilerConfig;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = include_bytes!("../../target/wasm32-unknown-unknown/release/wasm_test.wasm");
    // let wasm_bytes = include_bytes!("../../f32.wast");

    // Create a Store.
    let mut config = Singlepass::default();
    config.push_middleware(float_middleware::FloatMiddleware::new());
    config.push_middleware(std::sync::Arc::new(gatekeeper::Gatekeeper::default()));
    // config.push_middleware(print_middleware::PrintMiddleware::new());
    let store = Store::new(&Universal::new(config).engine());

    println!("Compiling module...");

    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;
    // Supply replacement functions for float operations
    let import_object = float_middleware::create_imports(&store);

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    // Call wasm function
    let wasm_fn = instance
        .exports
        .get_function("test_f32")?
        .native::<(u32, u32), u32>()?;
    // .native::<(f32, f32), f32>()?;

    println!("Calling `test_f32` function...");
    let result = wasm_fn.call(-2147483648i32 as u32, -2147483648i32 as u32)?;
    println!("Results of `test_f32`: {:?}", result);
    let expected = crate::math::add_f32(-2147483648i32 as u32, -2147483648i32 as u32);
    println!("Expected: {:?}", expected);

    assert_eq!(result, expected);
    // let result = wasm_fn.call(2.0f32, 1.5f32)?;
    // println!("Results of `test_f32`: {:?}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::wast::Wast;
    use wasmer_wast::spectest_importobject;

    use super::*;

    #[test]
    fn test() {
        // Create a Store.
        let mut config = Singlepass::default();
        config.push_middleware(float_middleware::FloatMiddleware::new());
        // config.push_middleware(std::sync::Arc::new(gatekeeper::Gatekeeper::default()));
        let store = Store::new(&Universal::new(config).engine());
        // combine the spectest imports with the float imports
        let mut import_object = float_middleware::create_imports(&store);
        let spectest_utils = spectest_importobject(&store)
            .get_namespace_exports("spectest")
            .unwrap();
        import_object.register("spectest", spectest_utils);

        let mut wast = Wast::new(store, import_object);
        println!("{:?}", std::fs::read_dir(".").unwrap().collect::<Vec<_>>());
        // wast.run_buffer(test, wast);
        wast.run_buffer(
            Path::new("f32.wast"),
            include_bytes!("../../tests/f32.wast"),
        )
        .unwrap();
    }
}
