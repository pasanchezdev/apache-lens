use anyhow::{anyhow, Result};
use chrono::DateTime;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

use crate::core::models::LogEntry;

static RE: OnceLock<Regex> = OnceLock::new();

fn regex() -> &'static Regex {
    RE.get_or_init(|| {
        Regex::new(
            r#"^(\S+) \S+ (\S+) \[([^\]]+)\] "(\S+) (\S+) (\S+)" (\d{3}) (\d+|-)(?:\s"([^"]*)")?(?:\s"([^"]*)")?"#,
        )
        .unwrap()
    })
}

pub fn parse_line(line: &str) -> Result<LogEntry> {
    let caps = regex()
        .captures(line)
        .ok_or_else(|| anyhow!("unmatched line: {line}"))?;

    let user = match &caps[2] {
        "-" => None,
        u   => Some(u.to_string()),
    };

    let timestamp = DateTime::parse_from_str(&caps[3], "%d/%b/%Y:%H:%M:%S %z")
        .map_err(|e| anyhow!("invalid date '{}': {e}", &caps[3]))?;

    let bytes = match &caps[8] {
        "-" => 0,
        b   => b.parse()?,
    };

    Ok(LogEntry {
        ip:         caps[1].to_string(),
        user,
        timestamp,
        method:     caps[4].to_string(),
        path:       caps[5].to_string(),
        protocol:   caps[6].to_string(),
        status:     caps[7].parse()?,
        bytes,
        referer:    caps.get(9).map(|m| m.as_str().to_string()),
        user_agent: caps.get(10).map(|m| m.as_str().to_string()),
    })
}

pub fn parse_file(path: &str) -> Result<Vec<LogEntry>> {
    let reader = BufReader::new(File::open(path)?);
    Ok(reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| parse_line(&l).ok())
        .collect())
}
