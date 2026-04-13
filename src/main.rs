mod cli;
mod commands;
mod config;
mod core;
mod discovery;
mod i18n;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use config::resolve_path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init                        => commands::init::run(),
        Command::Parse { file }              => commands::parse::run(&resolve_path(file)?),
        Command::Stats { file }              => commands::stats::run(&resolve_path(file)?),
        Command::Filter { file, ip, code, method } => {
            commands::filter::run(&resolve_path(file)?, ip.as_deref(), code, method.as_deref())
        }
        Command::Top { file, n }             => commands::top::run(&resolve_path(file)?, n),
        Command::Status { file, code, live } => commands::status::run(&resolve_path(file)?, code, live),
        Command::Export { file, output }     => commands::export::run(&resolve_path(file)?, &output),
    }
}
