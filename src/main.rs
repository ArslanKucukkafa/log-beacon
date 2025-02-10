use std::env::Args;
use std::io::Write;
use clap::Parser;
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
use services::help_service;

mod channel_example;
mod log_reader;
mod lru_cache;

fn main() {
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
        let args = shlex::split(line).expect("error: Invalid quoting");

        match Cli::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.command {
                    Commands::SocketServer(server) => {
                        println!("Starting WebSocket server with state: {:?}", "Running");
                    }
                    Commands::Level(level) => {
                        println!("Modifying log level. Add: {:?}, Remove: {:?}", "INFO", "DEBUG");
                    }
                    Commands::Suspend(suspend) => {
                        println!("Suspending logging for {:?} named {}", "SERVICE", "SERVICE_NAME");
                    }
                    Commands::Condition(condition) => {
                        println!("Adding condition for {:?} named {}", "SERVICE", "SERVICE_NAME");
                    }
                    Commands::Tag(tag) => {
                        println!("Adding tag '{}' for {:?} named {}", "IGNORE", "SERVICE", "SERVICE_NAME");
                    }
                    Commands::Configurations => {
                        help_service::show_help();
                        println!("Getting current LogBeacon configurations");
                    }
                    Commands::Exit => {
                        // ctrl + c || exit || quit durumunda uygulamadan çıkış yapar
                        println!("Exiting application");
                        break;
                    }
                    Commands::Clear => {
                        clearscreen::clear().unwrap();
                    }
                }
            }

            Err(e) => {
                e.print().expect("Command not found");

                continue;
            },
        };
    }
}