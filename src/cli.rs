use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "applogs", version, about = "Apache log analyzer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Initialize applogs and configure the log path")]
    Init,
    #[command(about = "Parse a log file and display its entries")]
    Parse {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
    },
    #[command(about = "Show general statistics: hits, unique IPs, HTTP codes")]
    Stats {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
    },
    #[command(about = "Filter entries by IP, HTTP code or method")]
    Filter {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
        #[arg(long, help = "Filter by IP address")]
        ip: Option<String>,
        #[arg(long, help = "Filter by HTTP status code")]
        code: Option<u16>,
        #[arg(long, help = "Filter by HTTP method (GET, POST, ...)")]
        method: Option<String>,
    },
    #[command(about = "Show top N IPs, paths and user-agents")]
    Top {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
        #[arg(short, long, default_value_t = 10, help = "Number of results to show")]
        n: usize,
    },
    #[command(about = "Show detailed view of each log entry")]
    Status {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
        #[arg(short, long, help = "Filter by HTTP status code")]
        code: Option<u16>,
        #[arg(short, long, help = "Watch log file in real time")]
        live: bool,
    },
    #[command(about = "Export results to a JSON file")]
    Export {
        #[arg(short, long, help = "Path to the Apache log file (overrides config)")]
        file: Option<String>,
        #[arg(short, long, help = "Output JSON file path")]
        output: String,
    },
}
