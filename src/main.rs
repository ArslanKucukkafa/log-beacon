mod log_model;
mod channel_example;

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Command line application starting... 🚀");

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
                    let level = log_levels.find(&log).map(|m| m.as_str());
                    if let Some(level) = level {
                        println!("[{}] {}", level, log);
                    } else {
                        println!("{}", log);
                    }
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
                if let Ok(log) = line {
                    let level = log_levels.find(&log).map(|m| m.as_str()).unwrap_or("ERROR");
                    eprintln!("[{}] {}", level, log);
                }
            }
        })
    };

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();
}
