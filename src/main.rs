use std::ascii::AsciiExt;
use std::env::Args;
use std::io;
use std::io::Write;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use colored::Colorize;
use log::logger;
use comfy_table::Table;

mod models;
use models::log_model;
use models::shell_model::{Commands,Cli};

mod services;
use services::server;
use services::tag_service;
use services::condition_service;
use services::suspend_service;
use services::configuration_service;
mod utils;
use utils::completion;

mod channel_example;
mod log_reader;
mod lru_cache;

fn main() {

    configuration_service::get_config();
    println!("-------------------------------------------------");

    let cli = Cli::parse();

    // Completion işlemi varsa sadece onu çalıştır ve çık
    if let Some(shell) = cli.completion {
        let mut cmd = Cli::command();
        cmd.set_bin_name("log-beacon");
        completion::generate_completion(shell, &mut cmd);
        return;
    }

    // Diğer komutlar için CLI döngüsünü başlat
    println!("Command line application starting... 🚀");
    start_cli();
}

// lru_cache::sample_cache()


/*
println!("Command line application starting... 🚀");
    log_reader::start_read();*/

fn start_cli() {
    loop {
        print!("👁️‍🗨️>>> "); // Print the prompt
        std::io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is displayed immediately

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();

        // Check for exit or quit commands
        if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
            println!("Exiting the log-beacon CLI... 👋");
            break;
        }

        if line.eq_ignore_ascii_case("clear"){
            clearscreen::clear().unwrap();
            continue;
        }

        let args = shlex::split(line).expect("error: Invalid quoting");

        match Cli::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.command {
                    Some(Commands::SocketServer(server)) => {
                        println!("Starting WebSocket server with state: {:?}", "Running");
                    }
                    Some(Commands::Level(level)) => {
                        println!("Modifying log level. Add: {:?}, Remove: {:?}", "INFO", "DEBUG");
                    }
                    Some(Commands::Suspend(suspend)) => {
                        println!("Suspending logging for {:?} named {}", "SERVICE", "SERVICE_NAME");
                    }
                    Some(Commands::Condition(condition)) => {
                        println!("Adding condition for {:?} named {}", "SERVICE", "SERVICE_NAME");
                    }
                    Some(Commands::Tag(tag)) => {
                        println!("Adding tag '{}' for {:?} named {}", "IGNORE", "SERVICE", "SERVICE_NAME");
                    }
                    Some(Commands::Configurations) => {
                        println!("Getting current LogBeacon configurations");
                    }
                    _ => {}
                }
            }

            Err(e) => {
                e.print().expect("Command not found");

                continue;

                /*                println!("{}", format!("Command not found {}", line).red().bold());
                help_service::show_help();
                continue;*/
            },
        };

    }
}