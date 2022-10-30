use anyhow::*;

mod merge;
mod traversal;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let input = args
        .nth(1)
        .ok_or_else(|| anyhow!("must provide the input wasm file as the first argument"))?;
    let output = args
        .next()
        .ok_or_else(|| anyhow!("must provide the output wasm file as the second argument"))?;

    let module = walrus::Module::from_file(&input)?;
    let float_module1 = walrus::Module::from_buffer(include_bytes!("../../float.wasm"))?;
    // let mut float_module2 = walrus::Module::from_buffer(include_bytes!("../../berkeley.wasm"))?;

    let mut module = merge::merge_modules(module, float_module1)?;

    let wasm = module.emit_wasm();
    std::fs::write(output, wasm)?;
    Ok(())
}
