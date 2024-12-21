use crate::Command::*;
use clap::{Parser, Subcommand};
use rogue_logging::LoggerBuilder;
use std::process::ExitCode;
use rogue_tremolo::{pull_command, push_command};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Pull {
        client: String,
        category: Option<String>,
    },
    Push {
        client: String,
        category: Option<String>,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let _logger = LoggerBuilder::new().create();
    let cli = Cli::try_parse().unwrap_or_else(|e| e.exit());
    let result = match cli.command {
        Pull { client, category } => pull_command(client, category).await,
        Push { client, category } => push_command(client, category).await,
    };
    result.unwrap_or_else(|e| {
        e.log();
        ExitCode::FAILURE
    })
}
