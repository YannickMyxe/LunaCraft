use std::{fs::{self, OpenOptions}, io::Write};

use crate::{config, files};

pub fn init(config_file: &str, files: Vec<String>, output: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Patching files...");

    config::create_file(config_file)?;
    println!("Config file created: {}", config_file);
    
    let mut disabled = Vec::new();
    let mut mods = Vec::new();

    for file in files {
        match file.split(".").last() {
            Some("disabled") => disabled.push(file.clone()),
            Some(_) => mods.push(file.clone()),
            None => eprintln!("Unsupported file type for file: {}", file.clone()),
        }
    }

    files::init(&disabled, &mods, output)
}