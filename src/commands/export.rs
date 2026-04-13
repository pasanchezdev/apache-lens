use anyhow::Result;

pub fn run(file: &str, output: &str) -> Result<()> {
    println!("[mock] export → {file} ⟶ {output}");
    Ok(())
}
