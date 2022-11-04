use anyhow::*;

mod replace;

const SOFTFLOAT_PREFIX: &str = "__wasm_soft_float_";

fn main() -> Result<()> {
    let mut args = std::env::args();
    let input = args
        .nth(1)
        .ok_or_else(|| anyhow!("must provide the input wasm file as the first argument"))?;
    let output = args
        .next()
        .ok_or_else(|| anyhow!("must provide the output wasm file as the second argument"))?;

    let mut module = walrus::Module::from_file(&input)?;

    // TODO: also check if it even has float operations first
    if !module
        .exports
        .iter()
        .any(|e| e.name.starts_with(SOFTFLOAT_PREFIX))
    {
        bail!("Could not find soft float operations in input module!\r\nPlease include the wasm-soft-floats crate as a dependency and include `pub use wasm_soft_floats::*;` somewhere in your crate.");
    }

    replace::replace_float_operations(&mut module)?;

    walrus::passes::gc::run(&mut module);

    let wasm = module.emit_wasm();
    std::fs::write(output, wasm)?;
    Ok(())
}
