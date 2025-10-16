use std::{fs, path::PathBuf, str::FromStr};

pub fn create_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = PathBuf::from_str(&file_path)?;
    if !config.exists() {
        fs::File::create(&config)?;
        Ok(())
    } else {
        Err("Config file already exists".into())
    }
}


pub fn exists(file_path: &str) -> bool {
    let config = PathBuf::from_str(&file_path).expect("Failed to parse config path");
    config.exists()
}