use anyhow::Result;

use crate::core::parser::parse_file;

pub fn run(file: &str, ip: Option<&str>, code: Option<u16>, method: Option<&str>) -> Result<()> {
    let entries = parse_file(file)?;

    let filtered: Vec<_> = entries
        .iter()
        .filter(|e| ip.map_or(true, |v| e.ip == v))
        .filter(|e| code.map_or(true, |v| e.status == v))
        .filter(|e| method.map_or(true, |v| e.method.eq_ignore_ascii_case(v)))
        .collect();

    if filtered.is_empty() {
        println!("\n  Sin resultados para los filtros aplicados.\n");
        return Ok(());
    }

    println!();
    println!("{}", "─".repeat(50));

    for entry in &filtered {
        println!("  {} {} → {}  {}  {} bytes",
            entry.timestamp.format("%d/%m/%Y  %H:%M:%S"),
            entry.ip,
            entry.path,
            entry.status,
            entry.bytes,
        );
    }

    println!("{}", "─".repeat(50));
    println!("  {} resultado(s)", filtered.len());

    if let Some(v) = ip     { println!("  IP       {v}"); }
    if let Some(v) = code   { println!("  Código   {v}"); }
    if let Some(v) = method { println!("  Método   {v}"); }

    println!();
    Ok(())
}
