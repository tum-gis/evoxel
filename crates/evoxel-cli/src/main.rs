mod arguments;
mod commands;

use crate::arguments::{Arguments, Commands};
use clap::Parser;
use std::path::{Path, PathBuf};

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Test {
            input_directory_path,
            output_directory_path,
        } => {
            let input_directory_path = Path::new(input_directory_path).canonicalize().unwrap();
            let output_directory_path = PathBuf::from(output_directory_path);

            commands::test::run(input_directory_path, output_directory_path);
        }
    };
}
