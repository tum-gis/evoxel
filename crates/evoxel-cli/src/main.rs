mod cli;
mod commands;
mod error;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test {
            input_directory_path,
            output_directory_path,
        } => {
            commands::test::run(input_directory_path.canonicalize()?, output_directory_path)?;
        }
    };

    Ok(())
}
