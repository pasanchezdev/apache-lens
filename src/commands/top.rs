use anyhow::Result;

pub fn run(file: &str, n: usize) -> Result<()> {
    println!("[mock] top {n} → {file}");
    Ok(())
}
