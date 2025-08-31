use crate::cli::{BucketCommands, Commands, FileCommands};
use crate::config::get_config_dir;
use crate::delete::delete_file;
use crate::download::download_file;
use crate::list_buckets::list_buckets;
use crate::list_files::list_files;
use crate::upload::upload_file;
use anyhow::{Result, bail};
use aws_sdk_s3::Client;
use clap::Parser;
use cli::Cli;
use config::Config;
use inquire::Confirm;
use std::fs;
use std::path::PathBuf;

mod cli;
mod config;
mod delete;
mod download;
mod list_buckets;
mod list_files;
mod s3_client;
mod upload;
mod util;

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
                println!("{}", list_buckets(&client).await?);
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
                println!("{}", list_files(&client, &bucket).await?);
            }
            FileCommands::Delete { bucket, key } => {
                match Confirm::new(&format!(
                    "Are you sure you want to delete the file {:?} from bucket {:?}? (y/n)",
                    key.clone(),
                    bucket.clone()
                ))
                .prompt()
                {
                    Ok(v) => {
                        if !v {
                            bail!("Aborting file deletion");
                        }

                        delete_file(&client, bucket, key.clone()).await?;

                        println!("Deleted {:?} successfully", key.clone());
                    }
                    Err(_) => {
                        bail!("There was an error when confirming file deletion");
                    }
                }
            }
            FileCommands::Download {
                bucket,
                key,
                location,
                override_filename,
            } => {
                download_file(&client, bucket, key.clone(), location, override_filename).await?;

                println!("Downloaded {:?} successfully", key.clone());
            }
            FileCommands::Upload {
                bucket,
                location,
                override_filename,
            } => {
                if override_filename.is_none() {
                    upload_file(
                        &client,
                        bucket,
                        location.clone().split('/').last().unwrap().to_string(),
                        location.clone(),
                    )
                    .await?;
                } else {
                    upload_file(
                        &client,
                        bucket,
                        override_filename.unwrap(),
                        location.clone(),
                    )
                    .await?;
                }

                println!(
                    "Uploaded {:?} successfully",
                    location.split('/').last().unwrap().to_string()
                );
            }
        },
    }

    Ok(())
}
