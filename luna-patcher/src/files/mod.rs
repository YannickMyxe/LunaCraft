use std::{fs::{self, OpenOptions}, io::Write};

pub fn init(disabled: &Vec<String>, mods: &Vec<String>, output: String) -> Result<(), Box<dyn std::error::Error>> {
    // Get dir from config or use default
    let dir = "./patches";

    if !fs::metadata(&dir).is_ok() {
        fs::DirBuilder::new()
            .recursive(false)
            .create(&dir)
            .expect("Failed to create directory");
    }

    let output = dir.to_owned() + "/" + &output.clone();

    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Clear the file if it exists
        .open(output.clone())
        .expect(format!("Cannot open file {}", output).as_str());

    write!(&mut data_file, "# Lunala Patcher\n").expect("Failed to write Header to file");

    if !disabled.is_empty() {
        write!(&mut data_file, "\n## Optional Files\n\n").expect("Failed to write section header to file");
        for file_name in disabled {
            write!(&mut data_file, "{}", format!("- [ ] {}\n", file_name))
                .expect("Failed to write to file, failed to add optional files");
        }
    }
    if !mods.is_empty() {
        write!(&mut data_file, "\n## Mod Files\n\n").expect("Failed to write section header to file");
        for file_name in mods {
            write!(&mut data_file, "{}", format!("- [x] {}\n", file_name))
                .expect("Failed to write to file, failed to add mod files");
        }
    }

    Ok(())
}