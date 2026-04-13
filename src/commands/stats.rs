use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::core::parser::parse_file;

fn bar(count: usize, max: usize, width: usize) -> String {
    let filled = if max == 0 { 0 } else { count * width / max };
    format!("{}{}", "#".repeat(filled), ".".repeat(width - filled))
}

fn format_bytes(bytes: u64) -> String {
    match bytes {
        b if b >= 1_073_741_824 => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
        b if b >= 1_048_576     => format!("{:.2} MB", b as f64 / 1_048_576.0),
        b if b >= 1_024         => format!("{:.2} KB", b as f64 / 1_024.0),
        b                       => format!("{b} bytes"),
    }
}

pub fn run(file: &str) -> Result<()> {
    let entries = parse_file(file)?;

    let total       = entries.len();
    let total_bytes: u64 = entries.iter().map(|e| e.bytes).sum();
    let unique_ips:  HashSet<&str> = entries.iter().map(|e| e.ip.as_str()).collect();

    let mut codes:   HashMap<u16, usize>  = HashMap::new();
    let mut methods: HashMap<&str, usize> = HashMap::new();

    for entry in &entries {
        *codes.entry(entry.status).or_insert(0)          += 1;
        *methods.entry(entry.method.as_str()).or_insert(0) += 1;
    }

    let ok      = entries.iter().filter(|e| e.status < 400).count();
    let errors  = entries.iter().filter(|e| e.status >= 400).count();

    println!();
    println!("{}", "─".repeat(50));
    println!("  General");
    println!("{}", "─".repeat(50));
    println!("  Total requests     {total}");
    println!("  Unique IPs         {}", unique_ips.len());
    println!("  Total transferred  {}", format_bytes(total_bytes));
    println!("  Successful         {ok}");
    println!("  Errors             {errors}");

    println!();
    println!("{}", "─".repeat(50));
    println!("  HTTP Status Codes");
    println!("{}", "─".repeat(50));

    let mut codes_sorted: Vec<_> = codes.iter().collect();
    codes_sorted.sort_by_key(|(code, _)| *code);
    let max_code = codes_sorted.iter().map(|(_, c)| **c).max().unwrap_or(1);

    for (code, count) in &codes_sorted {
        let label = match **code {
            c if c < 300 => "OK",
            c if c < 400 => "Redirect",
            c if c < 500 => "Client error",
            _            => "Server error",
        };
        println!("  {}  [{}]  {:>4}  {}", bar(**count, max_code, 20), label, count, code);
    }

    println!();
    println!("{}", "─".repeat(50));
    println!("  HTTP Methods");
    println!("{}", "─".repeat(50));

    let mut methods_sorted: Vec<_> = methods.iter().collect();
    methods_sorted.sort_by(|a, b| b.1.cmp(a.1));
    let max_method = methods_sorted.iter().map(|(_, c)| **c).max().unwrap_or(1);

    for (method, count) in &methods_sorted {
        println!("  [{}]  {:>4}  {}", bar(**count, max_method, 20), count, method);
    }

    println!();

    Ok(())
}
