use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Buckets {
        #[command(subcommand)]
        commands: BucketCommands,
    },
    Files {
        #[command(subcommand)]
        commands: FileCommands,
    },
    Init {},
}

#[derive(Subcommand, Debug)]
pub enum BucketCommands {
    List,
    Create {
        #[arg(required = true)]
        name: String,

        #[arg(required = true)]
        region: String,
    },
    Delete {
        #[arg(required = true)]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum FileCommands {
    List {
        #[arg()]
        bucket: String,
    },
    Delete {
        #[arg(required = true)]
        bucket: String,

        #[arg()]
        key: String,

        #[arg(short)]
        force: bool,
    },
    Download {
        #[arg(required = true)]
        bucket: String,

        #[arg()]
        key: String,

        #[arg()]
        location: String,

        #[arg(short, long)]
        override_filename: Option<String>,
    },
    Upload {
        #[arg(required = true)]
        bucket: String,

        #[arg()]
        location: String,

        #[arg(short, long)]
        override_filename: Option<String>,
    },
}
