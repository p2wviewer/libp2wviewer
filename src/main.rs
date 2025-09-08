mod cli;
mod encrypt;
mod decrypt;
mod crypto;
mod header;
mod image_ops;

use cli::{Cli, Commands};
use clap::Parser;
use log::{info, error};
use std::process;

fn main() {
    let args = Cli::parse();

    let log_level = match args.verbose {
        0 => "warn",
        1 => "info",
        _ => "debug",
    };
    std::env::set_var("RUST_LOG", log_level);
    env_logger::init();

    match args.command {
        Commands::Encrypt { input, output, password, password_file, split } => {
            info!("Encrypt command selected");
            let opts = encrypt::EncryptOptions {
                input_path: input.into(),
                output_path: output.into(),
                password,
                password_file,
                split: Some(split),
            };
            if let Err(e) = encrypt::run(opts) {
                error!("Encryption failed: {:?}", e);
                process::exit(1);
            }
        }

        Commands::Decrypt { input, output, password, password_file } => {
            info!("Decrypt command selected");
            let opts = decrypt::DecryptOptions {
                input_path: input.into(),
                output_path: output.into(),
                password,
                password_file,
            };
            if let Err(e) = decrypt::run(opts) {
                error!("Decryption failed: {:?}", e);
                process::exit(1);
            }
        }
    }
}