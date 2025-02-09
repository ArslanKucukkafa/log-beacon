use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use regex::Regex;
use crate::log_model::{LogLevel, LogModel};
use chrono::{DateTime, FixedOffset};

pub fn start_read(){

    let mut child = Command::new("java")
        .arg("-jar")
        .arg("/Users/arslankucukkafa/Desktop/log-producer-app/target/log-producer-app-0.0.1-SNAPSHOT.jar")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute process ❌");


    /// FIXME: Burda stdout ve stderr arasında arasında bir async sorunumuz var. Stacktrace mesajı stderr den alınırken, stacktrace mesajının geri kalanı stodout'dan çıkıyor.
    /// ## ÇÖZÜM:
    /// - stdout ve stderr'ı tek bir channel üzerinde okumalıyız.


    let stdout = Arc::new(Mutex::new(child.stdout.take().expect("Stdout okunamıyor!")));
    let stderr = Arc::new(Mutex::new(child.stderr.take().expect("Stderr okunamıyor!")));


    let log_levels = Arc::new(Regex::new(r"(?i)\b(INFO|ERROR|WARN|DEBUG|TRACE|STACKTRACE)\b").unwrap());

    let stdout_thread = {
        let stdout = Arc::clone(&stdout);
        let log_levels = Arc::clone(&log_levels);
        thread::spawn(move || {
            let mut stdout = stdout.lock().unwrap();
            let reader = BufReader::new(&mut *stdout);
            for line in reader.lines() {
                if let Ok(log) = line {
                    let log = parse_log(&log);
                    println!("{:?}", log);
                }
            }
        })
    };



    let stderr_thread = {
        let stderr = Arc::clone(&stderr);
        let log_levels = Arc::clone(&log_levels);
        thread::spawn(move || {
            let mut stderr = stderr.lock().unwrap();
            let reader = BufReader::new(&mut *stderr);
            for line in reader.lines() {
                let log = parse_log(&line.unwrap());
                println!("{:?}", log);
            }
        })
    };

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();
}

// convert to string log like as "2025-02-09T16:37:12.845+03:00  INFO 64920 --- [log-producer-app] [           main] o.apache.catalina.core.StandardEngine    : Starting Servlet engine: [Apache Tomcat/10.1.34]" convert to LogModel


fn parse_log(log: &str) -> Option<LogModel> {
    let re = Regex::new(r"(?x)
        (?P<time>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}\+\d{2}:\d{2})\s+
        (?P<level>INFO|WARN|WARNING|ERROR|DEBUG)\s+
        \d+\s+---\s+\[\s*(?P<service>.*?)\s*\]\s+\[.*?\]\s+
        (?P<class>[^\s:]+)\s+:\s+
        (?P<message>.*)
    ").unwrap();

    if let Some(caps) = re.captures(log) {
        let time = DateTime::parse_from_rfc3339(caps.name("time")?.as_str()).ok()?;
        let level = match caps.name("level")?.as_str() {
            "INFO" => LogLevel::INFO,
            "WARN" | "WARNING" => LogLevel::WARN,
            "ERROR" => LogLevel::ERROR,
            "DEBUG" => LogLevel::DEBUG,
            "TRACE" => LogLevel::TRACE,
            _ => return None,
        };
        let service = caps.name("service")?.as_str().to_string();
        let class = caps.name("class")?.as_str().to_string();
        let message = caps.name("message")?.as_str().to_string();

        Some(LogModel {
            level,
            time,
            service,
            class,
            message,
        })
    } else {
        None
    }
}