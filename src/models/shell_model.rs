use clap::{Subcommand, Args};
use clap_derive::{Args, Parser, Subcommand, ValueEnum};
use crate::log_model::LogLevel;

#[derive(Parser, Debug)]
#[command(name = "log-beacon")]
#[command(
    author = "Arslan Kucukkafa", 
    about = "CLI application for managing logs", 
    long_about = None, 
    version = "0.1.0",
    subcommand_help_heading = "Commands",
    arg_required_else_help = true,
    subcommand_required = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "log-server", about = "Start log websocket server")]
    SocketServer(SocketServer),

    #[command(name = "level", about = "Modify the log level (add/remove).")]
    Level(Level),

    #[command(name = "suspend", about = "Temporarily suspend logging for specific classes or services.")]
    Suspend(Suspend),

    #[command(name = "condition", about = "Filter logs from specific classes or services.")]
    Condition(Condition),

    #[command(name = "tag", about = "Belirli sınıf veya servislere etiket ekleme")]
    Tag(Tag),

    #[command(name = "config", about = "Get your current LogBeacon configurations")]
    Configurations,

    #[command(name = "exit", about = "Exit the application")]
    Exit,

    #[command(name = "clear", about = "Clear the screen")]
    Clear,
}

#[derive(Args, Debug)]
pub struct SocketServer {
    #[clap(short, long, help = "server process state")]
    pub state: ServerState,
}
// Bu enum SocketServer struct'ında kullanılmak üzere tanımlanmıştır.
#[derive(Clone, ValueEnum, Debug)]
pub enum ServerState {
    // run enumu string olarak port numarasını alır.
    RUN, //(String),
    Stop
}

#[derive(Args, Debug)]
pub struct Level {
    #[clap(help = "What kind of log level do you want to listen ")]
    pub add: LogLevel,
    #[clap(help = "What kind of log level don't you want to listen")]
    pub remove: LogLevel
}

#[derive(Args, Debug)]
pub struct Suspend {
    #[clap(help = "What kind of object do you want to suspend?")]
    pub filterable_objects: FilterableObjects,
    #[clap(help = "what object do you want to suspend with name?")]
    pub object_name: String,
}

#[derive(Args, Debug)]
pub struct Condition {
    #[clap(help = "What kind of object do you want to add condition?")]
    pub filterable_objects: FilterableObjects,
    #[clap(help = "what object do you want to add condition with name?")]
    pub object_name: String,
}

#[derive(Args, Debug)]
pub struct Tag {
    #[clap(value_enum, help = "what kind of object do you want to add tag?")]
    pub filterable_objects: FilterableObjects,
    
    #[clap(help = "what object do you want to add tag with name?")]
    pub object_name: String,
    
    #[clap(help = "what tag do you want to add?")]
    pub tag: String,
}

/// - `FilterableObjects` enumu benim condition, suspend ve tag komutlarında filtreleyebileceigm bir enumları içerir. Bu enum sayesinde daha net bir filterable object belirleyebilirim.

#[derive(Debug,Clone,ValueEnum)]
pub enum FilterableObjects {
    #[value(alias = "class")]
    CLASS,
    #[value(alias = "service")]
    SERVICE
}


