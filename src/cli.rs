use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {
        #[arg()]
        bucket: String,
    },
    Download {
        #[arg(required = true)]
        bucket: String,

        #[arg()]
        filename: Option<String>,

        #[arg()]
        location: Option<String>,

        #[arg(short, long)]
        override_filename: Option<String>,
    },
    Upload {
        #[arg(required = true)]
        bucket: String,

        #[arg()]
        filename: Option<String>,

        #[arg()]
        location: Option<String>,

        #[arg(short, long)]
        override_filename: Option<String>,
    },
}

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// pub struct Args {
//     #[arg(value_enum, required = true)]
//     pub method: Method,

//     #[arg(required = true)]
//     pub bucket: String,

//     #[arg()]
//     pub filename: Option<String>,

//     #[arg()]
//     pub location: Option<String>,

//     #[arg(short, long)]
//     pub override_filename: Option<String>
// }

// #[derive(ValueEnum, Clone, Debug)]
// pub enum Method {
//     List,
//     Download,
//     Upload,
// }
