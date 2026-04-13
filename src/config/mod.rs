use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_path: String,
    pub lang:     String,
}

pub fn config_path() -> PathBuf {
    dirs_next::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("applogs")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        return Err(anyhow!("No config found. Run 'applogs init' to set up."));
    }
    let content = fs::read_to_string(&path)?;
    toml::from_str(&content).map_err(Into::into)
}

pub fn save(config: &Config) -> Result<()> {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, toml::to_string(config)?)?;
    Ok(())
}

pub fn resolve_path(file: Option<String>) -> Result<String> {
    match file {
        Some(f) => Ok(f),
        None    => Ok(load()?.log_path),
    }
}
