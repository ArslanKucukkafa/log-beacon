use clap::Command;
use clap_complete::{generate, Shell};
use std::io;


use crate::models::shell_model::Cli;

pub fn generate_completion(shell: Shell, cmd: &mut Command) {
    let bin_name = cmd.get_name().to_string();
    generate(shell, cmd, bin_name, &mut io::stdout());
}