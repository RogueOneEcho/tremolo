use crate::Command::Pull;
use clap::{Parser, Subcommand};
use rogue_logging::LoggerBuilder;
use std::process::ExitCode;
use tremolo::pull_command;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Pull { client: String },
}

#[tokio::main]
async fn main() -> ExitCode {
    let _logger = LoggerBuilder::new().create();
    let cli = Cli::try_parse().unwrap_or_else(|e| e.exit());
    let result = match cli.command {
        Pull { client } => pull_command(client).await,
    };
    result.unwrap_or_else(|e| {
        e.log();
        ExitCode::FAILURE
    })
}
