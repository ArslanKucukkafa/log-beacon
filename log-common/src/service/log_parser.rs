use chrono::{DateTime};
use regex::Regex;
use crate::models::log_model::{LogModel, LogLevel};


/// convert to string log like as "2025-02-09T16:37:12.845+03:00  INFO 64920 --- [log-producer-app] [           main] o.apache.catalina.core.StandardEngine    : Starting Servlet engine: [Apache Tomcat/10.1.34]" convert to LogModel
/// log_readers::parse_log() fonksiyonu, verilen log satırını parse ederek LogModel döner.
/// fixme: ?? parse_log fonksiyonu sadece java logları için çalışır. Bu fonksiyonu genelleştirmek için ne yapabiliriz?

/// Ham string pattern alır, regex'i içerde oluşturur
pub fn parse_log(log: &str, pattern: &str) -> Result<LogModel, String> {
    // 1. Pattern'den raw string önekini kaldır
    let cleaned_pattern = pattern.trim_start_matches("r#\"").trim_end_matches('"');

    // 2. Regex'i doğrudan temizlenmiş pattern ile oluştur
    let re = Regex::new(cleaned_pattern)
        .map_err(|e| format!("Regex hatası: {}\nPattern: {}", e, cleaned_pattern))?;

    // 3. Mevut parsing mantığını koru
    let caps = re.captures(log).ok_or_else(|| {
        let error_msg = format!(
            "🛑 LOG PARSE HATASI\n[LOG] {}\n[PATTERN] {}",
            log, cleaned_pattern
        );
        eprintln!("{}", error_msg);
        error_msg
    })?;

    Ok(LogModel {
        time: DateTime::parse_from_rfc3339(caps.name("time").ok_or("Timestamp bulunamadı")?.as_str()).map_err(|e| format!("Timestamp parse error: {}", e))?,
        level: parse_level(caps.name("level").ok_or("Level bulunamadı")?.as_str())?,
        service: caps.name("service").ok_or("Service bulunamadı")?.as_str().to_string(),
        class: caps.name("class").map(|m| m.as_str().to_string()).unwrap_or_default(),
        message: caps.name("message").ok_or("Message bulunamadı")?.as_str().to_string(),
        tags: Vec::new(),
    })
}

/// Seviye parsingi için yardımcı fonksiyon
fn parse_level(level_str: &str) -> Result<LogLevel, String> {
    match level_str.to_uppercase().as_str() {
        "INFO" => Ok(LogLevel::INFO),
        "WARN" | "WARNING" => Ok(LogLevel::WARN),
        "ERROR" => Ok(LogLevel::ERROR),
        "DEBUG" => Ok(LogLevel::DEBUG),
        "TRACE" => Ok(LogLevel::TRACE),
        _ => Err(format!("Geçersiz log seviyesi: {}", level_str)),
    }
}
