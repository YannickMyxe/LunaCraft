use std::io::Write;
use std::{fs, path::PathBuf, str::FromStr};
use std::fs::{OpenOptions};

const DEFAULT_INIT_FILE: &str = "init.md";
const DEFAULT_PATCHES_DIR: &str = "patches";

pub fn create_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = PathBuf::from_str(&file_path)?;
    if !config.exists() {
        fs::File::create(&config)?;
        init_config(file_path);
        Ok(())
    } else {
        Err("Config file already exists".into())
    }
}

pub fn exists(file_path: &str) -> bool {
    let config = PathBuf::from_str(&file_path).expect("Failed to parse config path");
    config.exists()
}

fn init_config(file_path: &str) {
    let table = toml::toml! {
        [patches]
        intitFile = DEFAULT_INIT_FILE
        patchesDir = DEFAULT_PATCHES_DIR

        [pack]
        version = "1.0.0"
        packName = ""
        author = ""
        
        [mods]
        directory = ""
    };
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Clear the file if it exists
        .open(file_path)
        .expect(format!("Cannot open file {}", file_path).as_str());

    write!(&mut file, "{}", table.to_string()).expect("Failed to write default config");
}
