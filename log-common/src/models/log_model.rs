use chrono::{DateTime, FixedOffset};
use clap_derive::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

// DateTime serileştirme fonksiyonunu modül seviyesinde tanımlıyoruz
fn serialize_datetime<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(date.timestamp())
}

#[derive(Debug, PartialEq, Default, Clone, Serialize)]
pub struct LogModel {
    pub level: LogLevel,
    #[serde(serialize_with = "serialize_datetime")]
    pub time: DateTime<FixedOffset>,
    pub service: String,
    pub class: String,
    pub message: String,
    pub tags: Vec<String>,
}

#[derive(Debug, ValueEnum, Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "UPPERCASE")]
#[value(rename_all = "UPPERCASE")]
pub enum LogLevel {
    #[default]
    INFO,
    WARN,
    ERROR,
    DEBUG,
    TRACE,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
            LogLevel::TRACE => write!(f, "TRACE"),
        }
    }
}

pub struct LogTag {
    pub name: String,
    pub value: String,
}