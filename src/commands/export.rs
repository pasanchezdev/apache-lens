use anyhow::Result;
use std::fs::File;
use std::io::BufWriter;

use crate::core::parser::parse_file;

pub fn run(file: &str, output: &str) -> Result<()> {
    let entries = parse_file(file)?;

    let out = BufWriter::new(File::create(output)?);
    serde_json::to_writer_pretty(out, &entries)?;

    println!("\n  {} entradas exportadas a {output}\n", entries.len());
    Ok(())
}
