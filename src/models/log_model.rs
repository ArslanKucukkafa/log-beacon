use chrono::{DateTime, FixedOffset};
use clap_derive::ValueEnum;

#[derive(Debug)]
pub struct LogModel {
    pub level: LogLevel,
    pub time: DateTime<FixedOffset>,
    pub service: String,
    pub class: String,
    pub message: String,
}


#[derive(Debug, ValueEnum, Clone)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
    TRACE,
}

pub struct LogTag {
    pub name: String,
    pub value: String,
}