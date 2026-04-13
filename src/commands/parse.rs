use anyhow::Result;

use crate::core::parser::parse_file;

pub fn run(file: &str) -> Result<()> {
    let entries = parse_file(file)?;

    for entry in &entries {
        println!(
            "{} {} [{}] \"{} {}\" {} {}",
            entry.ip,
            entry.user.as_deref().unwrap_or("-"),
            entry.timestamp.format("%d/%b/%Y:%H:%M:%S %z"),
            entry.method,
            entry.path,
            entry.status,
            entry.bytes,
        );
    }

    println!("\n{} entries parsed.", entries.len());
    Ok(())
}
