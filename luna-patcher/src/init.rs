use std::{fs::{self, OpenOptions}, io::Write};

pub fn init(files: Vec<String>, output: String) {
    println!("Patching files...");

    let mut disabled = Vec::new();
    let mut mods = Vec::new();

    for file in files {
        match file.split(".").last() {
            Some("disabled") => disabled.push(file.clone()),
            Some(_) => mods.push(file.clone()),
            None => eprintln!(" - Unknown file type: {}", file.clone()),
        }
    }

    print(&disabled, &mods, output);
}

fn print(disabled: &Vec<String>, mods: &Vec<String>, output: String) {
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
            write!(&mut data_file, "{}", format!(" -[_] {}\n", file_name))
                .expect("Failed to write to file, failed to add disabled files");
        }
    }
    if !mods.is_empty() {
        write!(&mut data_file, "\n## Mod Files\n\n").expect("Failed to write section header to file");
        for file_name in mods {
            write!(&mut data_file, "{}", format!(" -[x] {}\n", file_name))
                .expect("Failed to write to file, failed to add mod files");
        }
    }
}