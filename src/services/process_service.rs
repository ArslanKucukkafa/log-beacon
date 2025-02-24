use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use nix::unistd::Pid;
use sysinfo::{ProcessesToUpdate, System};
use crate::models::config_model::Config;
use crate::services::configuration_service;
use crate::services::configuration_service::save_config;
use crate::models::log_model::{LogModel, LogLevel};
use crate::services::log_parser::parse_log;

#[derive(Debug)]
pub struct ProcessService {
    config: Config,
    child: Option<Child>,
    stop_flag: Arc<AtomicBool>,

}


// Bu servisin gorevi, verilen komutu calistirmak, durdurmak ve calisip calismadigini kontrol etmektir.
impl ProcessService {
    pub fn new() -> Self {
        Self {
            config: configuration_service::load_config().expect("Config yüklenemedi"),
            child: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run_process(&mut self, command: &str) -> mpsc::Receiver<LogModel> {
        let (tx, rx) = mpsc::channel();
        let stop_flag = Arc::clone(&self.stop_flag);

        // Pattern'i Arc ile sarmalalayıp paylaşılabilir hale getirelim
        let pattern = Arc::new(self.config.regexp.pattern.clone());

        let mut child = Command::new("/bin/bash")
            .arg("-l")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let stdout = child.stdout.take().unwrap();
        let tx_stdout = tx.clone();
        let stop_flag_stdout = Arc::clone(&stop_flag);
        let pattern_stdout = Arc::clone(&pattern);
        
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if stop_flag_stdout.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(line) = line {
                    match parse_log(&line, &pattern_stdout) {
                        Ok(log_model) => {
                            if let Err(e) = tx_stdout.send(log_model) {
                                eprintln!("Log gönderme hatası: {}", e);
                            }
                        }
                        Err(e) => {
                            let log = LogModel {
                                level: LogLevel::INFO,
                                time: chrono::Local::now().fixed_offset(),
                                service: "process".to_string(),
                                class: format!("parse_error: {}", e),
                                message: line,
                                tags: vec!["parse_error".to_string()]
                            };
                            if let Err(e) = tx_stdout.send(log) {
                                eprintln!("Log gönderme hatası: {}", e);
                            }
                        }
                    }
                }
            }
        });

        // Stderr için benzer işlem
        let stderr = child.stderr.take().unwrap();
        let tx_stderr = tx.clone();
        let stop_flag_stderr = Arc::clone(&stop_flag);
        let pattern_stderr = Arc::clone(&pattern);
        
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if stop_flag_stderr.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(line) = line {
                    match parse_log(&line, &pattern_stderr) {
                        Ok(mut log_model) => {
                            log_model.level = LogLevel::ERROR; // stderr için seviyeyi ERROR yap
                            if let Err(e) = tx_stderr.send(log_model) {
                                eprintln!("Log gönderme hatası: {}", e);
                            }
                        }
                        Err(_) => {
                            let log = LogModel {
                                level: LogLevel::ERROR,
                                time: chrono::Local::now().fixed_offset(),
                                service: "process".to_string(),
                                class: "stderr".to_string(),
                                message: line,
                                tags: vec![]
                            };
                            if let Err(e) = tx_stderr.send(log) {
                                eprintln!("Log gönderme hatası: {}", e);
                            }
                        }
                    }
                }
            }
        });

        // PID'i kaydet
        let pid = child.id();
        self.config.pid.process_pid = pid.to_string();
        self.save_config().expect("Error saving process PID");

        self.child = Some(child);

        rx
    }

    fn save_config(&self) -> Result<(), String> {
        // Config kaydetme işlemi
        Ok(())
    }

    /// Proses durdurma
    pub fn stop_process(&self) -> bool {
        let pid = &self.config.pid.process_pid; // Copy trait sayesinde move olmaz
        
        #[cfg(unix)] {
            use nix::sys::signal::{self, Signal};
            use nix::unistd::Pid;
            
            signal::kill(
                Pid::from_raw(pid.parse::<i32>().unwrap()),
                Signal::SIGTERM
            ).is_ok()
        }
        
        #[cfg(windows)] {
            use std::process::Command;
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            
            Command::new("taskkill")
                .creation_flags(CREATE_NO_WINDOW)
                .args(&["/F", "/T", "/PID", &pid.to_string()])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
    }

    /// Çalışma durumu kontrolü
    pub fn is_running(&self) -> bool {
        let pid = &self.config.pid.process_pid;
        
        #[cfg(unix)] {
            let mut sys = System::new();
            sys.refresh_processes(ProcessesToUpdate::All, true);
            sys.process(pid.parse().unwrap()).is_some()
        }
        
        #[cfg(windows)] {
            let mut sys = System::new();
            sys.refresh_processes(ProcessesToUpdate::All, true);
            sys.process(Pid::from(pid as usize)).is_some()
        }
    }
}