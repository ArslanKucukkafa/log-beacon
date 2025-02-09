use chrono::{DateTime, Utc};

pub struct LogModel {
    pub level: LogLevel,
    pub time: DateTime<Utc>,
    pub service: String,
    pub class: String,
    pub message: String,
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}