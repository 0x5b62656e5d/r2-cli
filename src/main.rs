use crate::buckets::create::create_bucket;
use crate::buckets::delete::delete_bucket;
use crate::buckets::list_buckets::list_buckets;
use crate::cli::{BucketCommands, Commands, FileCommands};
use crate::config::{Regions, init_config};
use crate::files::delete::delete_file;
use crate::files::download::download_file;
use crate::files::list_files::list_files;
use crate::files::upload::upload_file;
use crate::init::init_regions;
use crate::s3_client::build_client;
use crate::util::get_bucket_region;
use anyhow::{Result, bail};
use aws_sdk_s3::Client;
use clap::Parser;
use cli::Cli;
use config::Config;
use inquire::Confirm;

mod buckets;
mod cli;
mod config;
mod files;
mod init;
mod s3_client;
mod util;

#[::tokio::main]
async fn main() -> Result<()> {
    init_config()?;

    let config: Config = config::get_config()?;
    let regions: Regions = config::get_regions()?;

    let default_client: Client =
        s3_client::build_client(&config.default, "us-east-1".to_string()).await?;

    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Buckets { commands } => match commands {
            BucketCommands::List => {
                println!("{}", list_buckets(&default_client).await?);
            }
            BucketCommands::Create { name, region } => {
                create_bucket(&default_client, name.clone(), region).await?;

                println!("Created bucket {:?} successfully", name.clone());
            }
            BucketCommands::Delete { name } => {
                let client: Client =
                    build_client(&config.default, get_bucket_region(&regions, name.clone()))
                        .await?;

                match Confirm::new(&format!(
                    "Are you sure you want to delete the bucket {:?}? (y/n)",
                    name.clone()
                ))
                .prompt()
                {
                    Ok(v) => {
                        if !v {
                            bail!("Aborting bucket deletion");
                        }

                        delete_bucket(&client, name.clone()).await?;

                        println!("Deleted bucket {:?} successfully", name.clone());
                    }
                    Err(_) => {
                        bail!("There was an error when confirming bucket deletion");
                    }
                }
            }
        },
        Commands::Files { commands } => match commands {
            FileCommands::List { bucket } => {
                let client: Client =
                    build_client(&config.default, get_bucket_region(&regions, bucket.clone()))
                        .await?;

                println!("{}", list_files(&client, &bucket).await?);
            }
            FileCommands::Delete { bucket, key } => {
                let client: Client =
                    build_client(&config.default, get_bucket_region(&regions, bucket.clone()))
                        .await?;

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
                let client: Client =
                    build_client(&config.default, get_bucket_region(&regions, bucket.clone()))
                        .await?;

                download_file(&client, bucket, key.clone(), location, override_filename).await?;

                println!("Downloaded {:?} successfully", key.clone());
            }
            FileCommands::Upload {
                bucket,
                location,
                override_filename,
            } => {
                let client: Client =
                    build_client(&config.default, get_bucket_region(&regions, bucket.clone()))
                        .await?;

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
        Commands::Init {} => {
            init_regions().await?;
        }
    }

    Ok(())
}
