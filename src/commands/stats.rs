use anyhow::Result;
use std::collections::HashMap;

use crate::core::parser::parse_file;

pub fn run(file: &str) -> Result<()> {
    let entries = parse_file(file)?;

    let total = entries.len();
    let unique_ips: std::collections::HashSet<&str> =
        entries.iter().map(|e| e.ip.as_str()).collect();

    let mut codes: HashMap<u16, usize> = HashMap::new();
    let mut methods: HashMap<&str, usize> = HashMap::new();
    let total_bytes: u64 = entries.iter().map(|e| e.bytes).sum();

    for entry in &entries {
        *codes.entry(entry.status).or_insert(0) += 1;
        *methods.entry(entry.method.as_str()).or_insert(0) += 1;
    }

    println!("Total requests   : {total}");
    println!("Unique IPs       : {}", unique_ips.len());
    println!("Total bytes sent : {total_bytes}");

    println!("\nHTTP status codes:");
    let mut codes_sorted: Vec<_> = codes.iter().collect();
    codes_sorted.sort_by_key(|(code, _)| *code);
    for (code, count) in codes_sorted {
        println!("  {code}  →  {count}");
    }

    println!("\nHTTP methods:");
    let mut methods_sorted: Vec<_> = methods.iter().collect();
    methods_sorted.sort_by(|a, b| b.1.cmp(a.1));
    for (method, count) in methods_sorted {
        println!("  {method}  →  {count}");
    }

    Ok(())
}
