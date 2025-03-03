use std::collections::HashMap;
use std::fs;
use serde::Deserialize;
use serde::Serialize;
use crate::models::log_model::LogLevel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub log: LogConfig,
    pub socket: SocketConfig,
    pub suspend: SuspendConfig,
    pub condition: ConditionConfig,
    pub tag: TagConfig,
    pub regexp: RegexpConfig,
    pub pid: PIDConfig
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(rename = "levels")]
    pub levels: Vec<LogLevel>,
    #[serde(default)]
    pub enabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SocketConfig {
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendConfig {
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub services: Vec<String>,
    #[serde(default)]
    pub enabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionConfig {
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub services: Vec<String>,
    #[serde(default)]
    pub enabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagConfig {
    pub classes: HashMap<String, Vec<String>>,
    pub services: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub enabled: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegexpConfig {
    pub pattern: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PIDConfig {
    pub socket_pid: String,
    pub process_pid: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
