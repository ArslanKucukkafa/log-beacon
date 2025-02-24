use std::io::Write;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use rustyline::{error::ReadlineError, Editor};
use rustyline::history::{FileHistory, History};
use ctrlc;
use std::process;
use std::sync::Arc;
use shellwords;
use tokio::runtime::Runtime;

mod models;
use models::log_model;
use models::shell_model::Cli;

mod services;
mod utils;
mod channel_example;
mod lru_cache;
mod handlers;


fn main() {
    if std::env::args().len() > 1 {
        let rt = Runtime::new().expect("Runtime oluşturulamadı");

        rt.block_on(async {
            let cli = Cli::parse();
            if let Err(e) = handlers::command_handler::handle_command(cli.command).await {
                eprintln!("Komut işleme hatası: {}", e);
            }
        });
    } else {
        // Tokio runtime'ı içinde çalıştır
        tokio::runtime::Runtime::new()
            .expect("Runtime oluşturulamadı")
            .block_on(async {
                if let Err(e) = run().await {
                    eprintln!("Hata: {}", e);
                }
            });
    }
}



// lru_cache::sample_cache()


/*
println!("Command line application starting... 🚀");
    log_reader::start_read();*/

fn graceful_exit() -> ! {
    println!("\n{}", "Çıkış yapılıyor...".yellow());
    process::exit(0);
}

pub async fn run() -> Result<(), String> {
    ctrlc::set_handler(|| graceful_exit())
        .expect("Ctrl-C handler kurulamadı");

    // Tarihçe yapılandırması
    let mut rl = Editor::<(), FileHistory>::new()
        .expect("Readline başlatılamadı");
    rl.load_history("history.txt").ok(); // Geçmiş dosyası yoksa hata verme

    println!("{}", "› Log Beacon CLI'ya hoş geldiniz".green().bold());
    println!("{}", "  (Çıkış için 'exit' veya Ctrl+C)".dimmed());

    loop {
        match rl.readline(&*"»»» ".green().bold().to_string()) {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                rl.add_history_entry(input);

                // Özel komutlar
                match input.to_lowercase().as_str() {
                    "exit" | "quit" => graceful_exit(),
                    "clear" => {
                        clearscreen::clear().unwrap();
                        continue;
                    }
                    "history" => {
                        let history = rl.history();
                        if history.is_empty() {
                            println!("{}", "Henüz komut geçmişi yok".dimmed());
                        } else {
                            println!("{} ({} komut):", "Geçmiş".cyan().bold(), history.len());
                            for (i, entry) in history.iter().enumerate() {
                                println!("{:4}  {}", i + 1, entry.dimmed());
                            }
                        }
                        continue;
                    }
                    _ => ()
                }

                // Komut işleme
                let args = shellwords::split(input)
                    .map_err(|e| format!("Komut ayrıştırma hatası: {}", e))?;

                match Cli::try_parse_from(args) {
                    Ok(cli) => {
                        // Asenkron fonksiyonu .await ile çağırın
                        if let Err(e) = handlers::command_handler::handle_command(cli.command).await {
                            eprintln!("{}: {}", "Hata".red().bold(), e);
                        }
                    },
                    Err(e) => {
                        let mut msg = e.to_string();
                        msg += "\n\nÖzel Komutlar:\n  exit-quit-ctrlc\t\tÇıkış yap\n  clear\t\tEkranı temizle\n  history\tKomut geçmişini göster";
                        eprintln!("{}: {}", "Geçersiz komut".yellow().bold(), msg);
                    }
                }

            },
            Err(ReadlineError::Interrupted) => graceful_exit(), // Ctrl-C
            Err(ReadlineError::Eof) => graceful_exit(),         // Ctrl-D
            Err(err) => {
                return Err(format!("Girdi hatası: {}", err));
            }
        }
    }

    // Geçmişi kaydet
    rl.save_history("history.txt")
        .map_err(|e| format!("Geçmiş kaydetme hatası: {}", e))?;
    Ok(())
}
