/// ## Burda server işlemleri yapılacak
/// - Server başlatılacak `start_server()`
/// - Server durdurulacak `stop_server()`
/// - Server durumu kontrol edilecek `check_server_status()`
/// - Server için port adresi io olarak alınacak
/// - port adresi stop edilebilmesi için bir yerde saklanabilmeli. Yada isim verilebilmeli pid vs.

use crate::models::log_model::{LogLevel, LogModel};
use std::fs;
use std::io::{self, Write};
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use std::process::{Command, CommandArgs, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use serde_json;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use crate::services::configuration_service::{load_config, save_config};
use crate::services::process_service::ProcessService;
use std::sync::mpsc;

pub async fn start_listener(args: String, port: u16) -> Result<(), String> {
    let addr = format!("0.0.0.0:{}", port);
    println!("WebSocket sunucusu başlatılıyor: {}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Port bağlantı hatası: {}", e))?;

    let (log_tx, _) = broadcast::channel::<LogModel>(100);
    let mut process_service = ProcessService::new();

    // Log üretici thread'i
    let tx = log_tx.clone();
    let log_receiver = process_service.run_process(&args);
    
    // mpsc receiver'dan broadcast sender'a log aktarımı
    tokio::spawn(async move {
        loop {
            match log_receiver.try_recv() {
                Ok(log) => {
                    if let Err(e) = tx.send(log) {
                        eprintln!("Broadcast gönderme hatası: {}", e);
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Log üretici bağlantısı koptu");
                    break;
                }
            }
        }
    });

    println!("WebSocket sunucusu bağlantıları dinlemeye başladı");

    // WebSocket sunucusu
    while let Ok((stream, addr)) = listener.accept().await {
        println!("Yeni bağlantı geldi: {}", addr);
        
        let mut rx = log_tx.subscribe();
        
        tokio::spawn(async move {
            let ws_stream = match accept_async(stream).await {
                Ok(s) => {
                    println!("WebSocket bağlantısı başarıyla kuruldu: {}", addr);
                    s
                }
                Err(e) => {
                    eprintln!("WebSocket hatası {}: {}", addr, e);
                    return;
                }
            };

            let (mut writer, mut reader) = ws_stream.split();

            // Log gönderme döngüsü
            while let Ok(log) = rx.recv().await {
                match serde_json::to_string(&log) {
                    Ok(log_str) => {
                        if let Err(e) = writer.send(Message::Text(log_str.into())).await {
                            eprintln!("Log gönderme hatası: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("JSON serileştirme hatası: {}", e);
                    }
                }
            }
        });
    }

    // Port numarasını yazdır
    println!("Sunucu şu port'ta çalışıyor: {}", port);

    // Diğer mevcut kodlar...

    Ok(()) // Başarılı durumda boş bir Result döndür
}

// websocket'i durdurur
pub fn stop_server() -> bool {
    let mut config = load_config().map_err(|e| format!("Config yükleme hatası: {}", e)).unwrap();

    // PID dosyasını oku
    let pid = check_server_status(&config.socket.port);

    match pid {
        Some(pid) => {
            // Unix sistemlerde process'i sonlandır
            #[cfg(unix)]
            {
                if let Err(e) = Command::new("kill")
                    .arg("-9")
                    .arg(&pid.to_string())
                    .output() {
                    eprintln!("Process sonlandırılamadı: {}", e);
                    return false;
                }
            }

            // Windows sistemlerde process'i sonlandır
            #[cfg(windows)]
            {
                if let Err(e) = Command::new("taskkill")
                    .args(&["/F", "/PID", &pid.to_string()])
                    .output() {
                    eprintln!("Process sonlandırılamadı: {}", e);
                    return false;
                }
            }

            config.pid.process_pid = "".to_string();
            save_config(config).map_err(|e| format!("Config kaydetme hatası: {}", e)).unwrap();
            return true;
        }
        None => {
            eprintln!("Çalışan sunucu bulunamadı");
            false
        }
    }
}

// websocket'in çalışıp, çalışmadııgnı kontrol eder
pub fn check_server_status(port: &str) -> Option<String> {
    // lsof komutu ile port bilgisi için kontrol yap
    let output = Command::new("lsof")
        .arg("-i")
        .arg(format!(":{}", port)) // Portu kontrol et
        .arg("-t") // Sadece PID'yi döndür
        .output();

    // Komut başarılı çıktıysa PID döndür, aksi halde None
    if let Ok(output) = output {
        if !output.stdout.is_empty() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    } else {
        None
    }
}
