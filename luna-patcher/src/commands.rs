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
            let dir_files = read_dir(directory)?;
            let mut files: Vec<String> = Vec::new();
            for result in dir_files {
                let file = result.expect("Cannot read file").path();
                if let Some(name) = file.file_name() {
                    if !file.is_dir() {
                        files.push(name.to_string_lossy().to_string());
                    }
                }
            }
            patch::patch(files);
        },
    };
    Ok(())
}
