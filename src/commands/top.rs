use anyhow::Result;
use std::collections::HashMap;

use crate::core::parser::parse_file;

fn top_n(map: &HashMap<&str, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<_> = map.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(n);
    entries
}

fn print_ranking(title: &str, items: &[(String, usize)]) {
    println!("{title}");
    println!("{}", "─".repeat(50));
    for (i, (key, count)) in items.iter().enumerate() {
        println!("  {:>2}.  {:<35}  {} req", i + 1, key, count);
    }
    println!();
}

pub fn run(file: &str, n: usize) -> Result<()> {
    let entries = parse_file(file)?;

    let mut ips:    HashMap<&str, usize> = HashMap::new();
    let mut paths:  HashMap<&str, usize> = HashMap::new();
    let mut agents: HashMap<&str, usize> = HashMap::new();

    for entry in &entries {
        *ips.entry(entry.ip.as_str()).or_insert(0) += 1;
        *paths.entry(entry.path.as_str()).or_insert(0) += 1;
        if let Some(ref ua) = entry.user_agent {
            *agents.entry(ua.as_str()).or_insert(0) += 1;
        }
    }

    println!();
    print_ranking(&format!("Top {n} IPs"),         &top_n(&ips, n));
    print_ranking(&format!("Top {n} rutas"),        &top_n(&paths, n));
    print_ranking(&format!("Top {n} user-agents"),  &top_n(&agents, n));

    Ok(())
}
