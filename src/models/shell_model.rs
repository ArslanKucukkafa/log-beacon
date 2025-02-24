use clap::{Subcommand, Args};
use clap_complete::Shell;
use clap_derive::{Args, Parser, Subcommand, ValueEnum};
use clearscreen::ClearScreen;
use serde::{Deserialize, Serialize};
use crate::log_model::LogLevel;
use std::str::FromStr;

#[derive(Parser)]
#[command(
    name = "log-beacon",
    author = "Arslan Kucukkafa",
    about = "CLI application for managing logs", 
    long_about = "Log yönetimi için CLI aracı\n\nInteraktif modda çalıştırıldığında ek komutlar mevcuttur",
    version = "0.1.0",
    subcommand_help_heading = "Commands",
    subcommand_required = true,
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Listen(ListenerSubcommand),
    #[command(subcommand)]
    Level(LevelSubcommand),
    #[command(name = "suspend", about = "Temporarily suspend logging for specific classes or services.", subcommand)]
    Suspend(SuspendSubcommand),

    /// Koşul yönetimi
    #[command(subcommand)]  // Bu satır kritik!
    Condition(ConditionSubcommand),

    #[command(subcommand, about = "Tag operations")]
    Tag(TagSubcommand),

    #[command(name = "regex", about = "Parse your custom logs with your custom regex pattern", subcommand)]
    Regexp(RegexSubcommand),

    #[command(name = "config", about = "Get your current LogBeacon configurations")]
    Config,
}

#[derive(Subcommand, Debug)]
pub enum TagSubcommand {
    #[command(name = "add", about = "Add tag to object")]
    Add(TagAdd),
    #[command(name = "rm", about = "Remove tag from object")]
    Remove(TagRemove),
    #[command(name = "list", about = "List tags")]
    List,
    #[command(name = "clear", about = "Clear all tags")]
    Clear,
    #[command(name = "disable", about = "Disable tags")]
    Disable,
}

#[derive(Debug, Subcommand)]
pub enum ListenerSubcommand {
    #[command(name = "run", about = "start the listen")]
    Run(RunServer),
    #[command(name = "stop", about = "stop the listen")]
    Stop,
    #[command(name = "check", about = "check the server status")]
    Check
}

#[derive(Debug, Subcommand)]
pub enum LevelSubcommand {
    Add {
        #[arg(value_enum)]
        level: LogLevel,
    },
    Remove {
        #[arg(value_enum)]
        level: LogLevel,
    },
    List
}

#[derive(Debug, Subcommand)]
pub enum SuspendSubcommand {
    Add {
        #[command(flatten)]
        target: ObjectTarget
    },
    Remove {
        #[command(flatten)]
        target: ObjectTarget
    },
    List,
    Clear,
    Disable
}

#[derive(Debug, Subcommand)]
pub enum ConditionSubcommand {
    Add {
        #[command(flatten)]
        target: ObjectTarget
    },
    Remove {
        #[command(flatten)]
        target: ObjectTarget
    },
    List,
    Clear,
    Disable
}

#[derive(Args, Debug)]
pub struct TagAdd {
    #[clap(flatten)]  // İç içe struct'lar için
    pub object_target: ObjectTarget,

    #[clap(help = "Tag to add")]
    pub tag: String,
}

#[derive(Args, Debug)]
pub struct TagRemove {
    #[clap(flatten)]
    pub object_target: ObjectTarget,
    
    #[clap(help = "Tag to remove")]
    pub tag: String,
}

#[derive(Debug, Subcommand)]
pub enum RegexSubcommand {
    /// Yeni regex kuralı ekle
    Add {
        #[arg(short, long)]
        pattern: String
    },
    /// Regex kuralını test et
    Test {
        #[clap(long, value_parser, num_args = 1..)] // Tüm argümanları tek string al
        pattern: String,

        #[clap(long, value_parser, num_args = 1..)]
        sample_log: String
    },
}

#[derive(Debug, Args, Clone)]
pub struct RegexUpdate {
    #[arg(short, long, required = true)]
    pub old_pattern: String,
    
    #[arg(short, long, required = true)]
    pub new_pattern: String,
}

impl FromStr for RegexUpdate {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "=>").collect();
        if parts.len() != 2 {
            return Err("Geçersiz format. Kullanım: eski=>yeni".into());
        }
        Ok(Self {
            old_pattern: parts[0].trim().to_string(),
            new_pattern: parts[1].trim().to_string(),
        })
    }
}

/// - `ObjectType` enumu benim condition, suspend ve tag komutlarında filtreleyebileceigm bir enumları içerir. Bu enum sayesinde daha net bir filterable object belirleyebilirim.

#[derive(Debug,Clone,ValueEnum)]
pub enum ObjectType {
    #[value(alias = "class")]
    CLASS,
    #[value(alias = "service")]
    SERVICE,
    #[value(alias = "message")]
    MESSAGE
}

#[derive(Debug, Args)]
pub struct ObjectTarget {
    #[arg(short, long)]
    pub object_type: ObjectType,
    
    #[arg(short, long)]
    pub object_name: String,
}

#[derive(Args, Debug)]
pub struct RunServer {
    #[clap(
        help = "Port number to listen on",
        long_help = "Specifies the TCP port for the websocket server.\nUse values between 1024-65535 for user-level ports.",
    )]
    pub port: String,
    #[clap(help = "Command to execute")]
    pub args: String
}


