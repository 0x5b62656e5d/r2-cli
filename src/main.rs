use crate::cli::Commands;
use crate::config::get_config_dir;
use anyhow::{Result, bail};
use aws_sdk_s3::Client;
use clap::Parser;
use cli::Cli;
use config::Config;
use list_files::list_files;
use std::fs;
use std::path::PathBuf;

mod cli;
mod config;
mod list_files;
mod s3_client;

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

    let client: Client = s3_client::build_client(&config.default).await?;

    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::List { bucket } => {
            println!("{:?}", list_files(&client, &bucket).await?);
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
