use crate::cli::Commands;
use crate::config::get_config_dir;
use anyhow::{Result, bail};
use clap::Parser;
use cli::Cli;
use config::Config;
use std::fs;
use std::path::PathBuf;

mod cli;
mod config;
mod download;
mod list;
mod upload;

#[::tokio::main]
async fn main() -> Result<()> {
    let config_dir: PathBuf = get_config_dir();

    if !config_dir.is_dir() {
        fs::create_dir_all(&config_dir)?;
    }

    if let Ok(res) = fs::exists(config_dir.join("config.toml")) {
        if !res {
            fs::write(config_dir.join("config.toml"), "[default]\n")?;
        }
    }

    let config: Config = config::get_config()?;

    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::List { bucket } => {
            println!("List files: {bucket}");
        }
        Commands::Download {
            bucket,
            filename,
            location,
            override_filename,
        } => {
            println!("Download files: {bucket}, {filename:?}, {location:?}, {override_filename:?}");
        }
        Commands::Upload {
            bucket,
            filename,
            location,
            override_filename,
        } => {
            println!("Upload files: {bucket}, {filename:?}, {location:?}, {override_filename:?}");
        }
    }

    Ok(())
}
