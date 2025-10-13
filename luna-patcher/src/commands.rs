use std::{fs::read_dir, path::Path};

use clap::Parser;

use crate::patch;

/// Luna Patcher CLI
#[derive(Parser)]
#[command(name = "luna-patcher")]
#[command(about = "Created versions/patches based on your file changes", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Collect changes and create patch notes
    Patch {
        #[arg(value_name = "DIRECTORY")]
        directory: String,
    },
}

pub fn run(cli: Cli) -> std::io::Result<()> {
    match &cli.command {
        Commands::Patch { directory } => {
            let files = read_dir(directory)?; //.expect("Cannot read file directory");
            for x in files {
                let file = x.expect("Cannot read file").path();

                if let Some(name) = file.file_name() {
                    if file.is_dir() {
                        println!("Directory: {}", name.to_string_lossy());
                    } else {
                        println!("File: {}", name.to_string_lossy());
                    }
                }
            }
            patch::patch();
        },
    };
    Ok(())
}
