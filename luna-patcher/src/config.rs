use std::{fs, path::PathBuf, str::FromStr};



pub fn create(file_path: &str) {
    let config = PathBuf::from_str(&file_path).unwrap();
    if !config.exists() {
        eprintln!("Creating config {}", config.to_str().unwrap());
        fs::File::create(&config).expect("Failed to create config file");
    }
}


pub fn exists(file_path: &str) -> bool {
    let config = PathBuf::from_str(&file_path).expect("Failed to parse config path");
    config.exists()
}