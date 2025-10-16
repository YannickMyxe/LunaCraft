use std::{fs::read_dir, path::{PathBuf}, str::FromStr};

use clap::Parser;

use crate::{config, init};

/// Luna Patcher CLI
#[derive(Parser)]
#[command(name = "luna-patcher")]
#[command(
    about = "Created versions/patches based on your file changes",
    version = "1.0"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Collect changes and create patch notes
    Init {
        #[arg(value_name = "DIRECTORY")]
        directory: String,

        /// Optional output file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<String>,
    },
    Patch {
        #[arg(value_name = "DIRECTORY")]
        directory: String,

        /// Optional output file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<String>,
    },
}

pub fn run(cli: Cli, config: &str) -> Result<(), String> {
    match &cli.command {
        Commands::Init { directory, output } => {
            let dir_files = read_dir(directory).map_err(|_| "Failed to read directory")?;
            let files_vec: Vec<_> = dir_files
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "Failed to read files")?;
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
            init::init(config, files, output.clone().unwrap_or("init.md".to_string()));
        }
        Commands::Patch { directory, output } => {
            let config = PathBuf::from_str(&config).unwrap();
            println!(
                "Patching directory: {} with config: {:?}",
                directory, config
            );
            if !config.exists() {
                eprintln!("{} does not exist", config.to_str().unwrap());
                Err(String::from("Config file does not exist"))?;
            }

            config::create(config.to_str().unwrap());
        }
    };
    Ok(())
}
