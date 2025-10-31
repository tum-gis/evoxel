use clap::ValueHint;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run some tests
    Test {
        /// Input directory
        #[clap(short, long, value_hint = ValueHint::DirPath)]
        input_directory_path: PathBuf,

        /// Output directory
        #[clap(short, long, value_hint = ValueHint::DirPath)]
        output_directory_path: PathBuf,
    },
}
