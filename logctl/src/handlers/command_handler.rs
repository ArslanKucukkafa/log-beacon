use std::process;
use crate::models::shell_model::{Commands, ListenerSubcommand, RegexSubcommand, LevelSubcommand, SuspendSubcommand, ConditionSubcommand, TagSubcommand};
use crate::services::{condition_service, configuration_service, regexp_service, suspend_service};
use crate::services::tag_service;
use crate::services::listener::{check_server_status,start_listener,stop_server};
use crate::Cli;
use crate::services::regexp_service::test_regexp;
use crossterm::style::Stylize;
use futures_util::TryFutureExt;
use tokio;

pub async fn handle_command(command: Commands) -> Result<(), String> {
    match command {
        Commands::Listen(listen_command) => handle_listener(listen_command).await, // .await ekledik
        Commands::Level(level_command) => handle_level(level_command),
        Commands::Suspend(suspend_command) => handle_suspend(suspend_command),
        Commands::Condition(condition_command) => handle_condition(condition_command),
        Commands::Tag(tag_command) => handle_tag(tag_command),
        Commands::Regexp(regexp_command) => handle_regexp(regexp_command),
        Commands::Config => {
            let config = configuration_service::load_config()?;
            configuration_service::display_config(&config);
            Ok(())
        }
        _ => Err("Command not implemented".to_string()),
    }
}

pub async fn handle_listener(command: ListenerSubcommand) -> Result<(), String> {
    match command {
        ListenerSubcommand::Run(listen_command) => {
            let port = listen_command.port.parse::<u16>()
                .map_err(|_| "Invalid port number".to_string())?;
            
            start_listener(listen_command.args, port).await
        },
        ListenerSubcommand::Stop => {
            match stop_server() {
                true => Ok(()),
                false => Err("Sunucu durdurma işlemi başarısız oldu".to_string())
            }
        },
        ListenerSubcommand::Check => {
            let config = configuration_service::load_config()
                .map_err(|e| format!("Konfigürasyon yüklenirken hata: {}", e))?;

            match check_server_status(&config.socket.port) {
                Some(pid) => {
                    println!("Sunucu çalışıyor. PID: {}", pid);
                    Ok(())
                },
                None => {
                    println!("Sunucu çalışmıyor.");
                    Ok(())
                }
            }
        }
    }
}

fn handle_level(level: LevelSubcommand) -> Result<(), String> {
    println!("Modifying log level. Add: {:?}, Remove: {:?}", "INFO", "DEBUG");
    Ok(())
}

fn handle_suspend(suspend: SuspendSubcommand) -> Result<(), String> {
    match suspend {
        SuspendSubcommand::Add { target } => {
            suspend_service::add_suspension(target.object_type, &target.object_name)?;
            Ok(())
        }
        SuspendSubcommand::Remove { target } => {
            suspend_service::remove_suspension(target.object_type, &target.object_name)?;
            Ok(())
        }
        SuspendSubcommand::List => {
            suspend_service::list_suspends()?;
            Ok(())
        }
        SuspendSubcommand::Clear => {
            suspend_service::clear_suspends()?;
            Ok(())
        }
        SuspendSubcommand::Disable => {
            suspend_service::disable_suspend_filter()?;
            Ok(())
        }
    }
}

fn handle_condition(condition: ConditionSubcommand) -> Result<(), String> {
    match condition {
        ConditionSubcommand::Add { target } => {
            condition_service::add_condition(target.object_type, &target.object_name)
        }
        ConditionSubcommand::Remove { target } => {
            condition_service::remove_condition(target.object_type, &target.object_name)
        }
        ConditionSubcommand::List => {
            condition_service::list_conditions()
        }
        ConditionSubcommand::Disable => {
            condition_service::disable_condition_filter()
        }
        ConditionSubcommand::Clear => {
            condition_service::clear_conditions()
        }
    }
}
fn handle_regexp(regexp: RegexSubcommand) -> Result<(), String> {
    match regexp {
        RegexSubcommand::Add { pattern } => {
            Ok(regexp_service::add_regexp(&pattern))
        }
        RegexSubcommand::Test { pattern, sample_log } => {
            match regexp_service::test_regexp(&pattern, &sample_log) {
                Ok(table) => {
                    println!("{}", table);
                    Ok(())
                },
                Err(e) => {
                    eprintln!("{}", e.red().bold());
                    Err("Regex test failed".to_string())
                }
            }
        }
    }
}

fn handle_tag(tag: TagSubcommand) -> Result<(), String> {
    match tag {
        TagSubcommand::Add(add_args) => {
            tag_service::add_tag(add_args.object_target.object_type, &add_args.object_target.object_name, &add_args.tag)?;
            Ok(())
        }
        TagSubcommand::Remove(remove_args) => {
            tag_service::remove_tag(remove_args.object_target.object_type, &remove_args.object_target.object_name, &remove_args.tag)?;
            Ok(())
        }
        TagSubcommand::List => {
            tag_service::list_tags();
            Ok(())
        }
        TagSubcommand::Clear => {
            tag_service::clear_tags()?;
            Ok(())
        }
        TagSubcommand::Disable => {
            tag_service::disable_tag_filter()?;
            Ok(())
        }
    }
}