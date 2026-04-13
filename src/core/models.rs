use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub ip:         String,
    pub user:       Option<String>,
    pub timestamp:  DateTime<FixedOffset>,
    pub method:     String,
    pub path:       String,
    pub protocol:   String,
    pub status:     u16,
    pub bytes:      u64,
    pub referer:    Option<String>,
    pub user_agent: Option<String>,
}
