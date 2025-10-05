use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "p2wviewer",
    version,
    about = "Encrypt and decrypt images into self-contained noise images",
)]
pub struct Cli {
    /// Main
    #[command(subcommand)]
    pub command: Commands,
    /// Verbose
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Split {
        /// Input file path
        #[arg(short = 'i', long)]
        input: String,

        /// Number of parts to split into
        #[arg(short = 'n', long, default_value = "2")]
        num_parts: u32,

        /// Delete the original fil
        #[arg(short = 'd', long)]
        delete_original: bool,
    },

    Merge {
        /// Input files (parts to merge)
        #[arg(short = 'i', long, value_delimiter = ',')]
        inputs: Vec<String>,

        /// Output file path
        #[arg(short = 'o', long)]
        output: String,
    },

    Encrypt {
        /// Input file path
        #[arg(short = 'i', long)]
        input: String,

        /// Output file path
        #[arg(short = 'o', long)]
        output: String,

        /// Pwd
        #[arg(short = 'p', long, group = "auth_method")]
        password: Option<String>,

        /// File to use as a password/key
        #[arg(long, group = "auth_method")]
        password_file: Option<String>,
    
        /// Number of blocks
        #[arg(short = 's', long, default_value = "1")]
        split: u32,
    },

    Decrypt {
        /// Input File or Dir
        #[arg(short = 'i', long)]
        input: String,

        /// Output file
        #[arg(short = 'o', long)]
        output: String,

        /// Pwd
        #[arg(short = 'p', long, group = "auth_method")]
        password: Option<String>,

        /// File used as the password/key
        #[arg(long, group = "auth_method")]
        password_file: Option<String>,
    },
}