use crate::cli::{BucketCommands, Commands, FileCommands};
use crate::config::get_config_dir;
use crate::list_buckets::{list_buckets, pretty_print};
use crate::list_files::list_files;
use anyhow::{Result, bail};
use aws_sdk_s3::Client;
use clap::Parser;
use cli::Cli;
use config::Config;
use std::fs;
use std::path::PathBuf;

mod cli;
mod config;
mod list_buckets;
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
        Commands::Buckets { commands } => match commands {
            BucketCommands::List => {
                pretty_print(list_buckets(&client).await?).await;
            }
            BucketCommands::Create { name } => {
                println!("Creating bucket: {name}");
            }
            BucketCommands::Delete { name } => {
                println!("Deleting bucket: {name}");
            }
        },
        Commands::Files { commands } => match commands {
            FileCommands::List { bucket } => {
                println!("{:?}", list_files(&client, &bucket).await?);
            }
            FileCommands::Download {
                bucket,
                filename,
                location,
                override_filename,
            } => {
                println!(
                    "Download files: {bucket}, {filename:?}, {location:?}, {override_filename:?}"
                );
            }
            FileCommands::Upload {
                bucket,
                filename,
                location,
                override_filename,
            } => {
                println!(
                    "Upload files: {bucket}, {filename:?}, {location:?}, {override_filename:?}"
                );
            }
        },
    }

    Ok(())
}
