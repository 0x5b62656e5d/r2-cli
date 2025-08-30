use anyhow::{Result, bail};
use clap::Parser;

mod cli;

use cli::Cli;
use crate::cli::Commands;

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::List { bucket } => {
            println!("List files: {bucket}")
        }
        Commands::Download {
            bucket,
            filename,
            location,
            override_filename,
        } => {
            println!("Download files: {bucket}, {filename:?}, {location:?}, {override_filename:?}")
        }
        Commands::Upload {
            bucket,
            filename,
            location,
            override_filename,
        } => {
            println!("Upload files: {bucket}, {filename:?}, {location:?}, {override_filename:?}")
        }
    }

    Ok(())
}
