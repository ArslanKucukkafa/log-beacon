use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use crate::models::log_model::LogLevel;
use crate::models::shell_model::Suspend;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub socket: SocketConfig,
    pub suspend: SuspendConfig,
    pub condition: ConditionConfig,
    pub tag: TagConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub levels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SocketConfig {
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct SuspendConfig {
    pub classes: Option<HashMap<String, String>>,
    pub services: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct ConditionConfig {
    pub classes: Option<HashMap<String, String>>,
    pub services: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct TagConfig {
    pub classes: Option<HashMap<String, String>>,
    pub services: Option<HashMap<String, String>>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
