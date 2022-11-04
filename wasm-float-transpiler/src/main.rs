use anyhow::*;

// mod merge;
mod replace;
// mod traversal;

const SOFTFLOAT_PREFIX: &str = "__wasm_soft_float_";

fn main() -> Result<()> {
    // let mut args = std::env::args();
    // let input = args
    //     .nth(1)
    //     .ok_or_else(|| anyhow!("must provide the input wasm file as the first argument"))?;
    // let output = args
    //     .next()
    //     .ok_or_else(|| anyhow!("must provide the output wasm file as the second argument"))?;
    let input = "./target/wasm32-unknown-unknown/release/test_contract.wasm";
    // let input = "./target/wasm32-unknown-unknown/release/wasm_test.wasm";
    // let input = "berkeley.wasm";
    let output = "test.wasm";

    let mut module = walrus::Module::from_file(&input)?;

    // TODO: also check if it even has float operations first
    if !module
        .exports
        .iter()
        .any(|e| e.name.starts_with(SOFTFLOAT_PREFIX))
    {
        bail!("Could not find soft float operations in input module!\r\nPlease include the wasm-soft-floats crate as a dependency and call the `wasm_soft_float::import_soft_floats!` macro in your crate root.");
    }

    replace::replace_float_operations(&mut module)?;

    // walrus::passes::gc::run(&mut module);

    let wasm = module.emit_wasm();
    std::fs::write(output, wasm)?;
    Ok(())
}
