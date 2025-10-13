use std::{fs::read_dir};

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

pub fn run(cli: Cli) -> Result<(), String> {
    match &cli.command {
        Commands::Patch { directory } => {
            let dir_files = read_dir(directory).map_err(|_| "Failed to read directory")?;
            let files_vec: Vec<_> = dir_files.collect::<Result<Vec<_>, _>>().map_err(|_| "Failed to read files")?;
            if files_vec.is_empty() {
                eprintln!("No files found in directory: {}", directory);
                return Err(String::from("Directory is empty"));
            }
            let mut files: Vec<String> = Vec::new();
            for entry in files_vec {
                let file = entry.path();
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
