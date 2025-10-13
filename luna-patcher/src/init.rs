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

    if !fs::metadata(output.clone()).is_ok() {
        // Create the file if it does not exist
        std::fs::File::create(output.clone()).expect("Failed to create output file");
    } else {
        std::fs::remove_file(output.clone()).expect("Failed to remove file");
        std::fs::File::create(output.clone()).expect("Failed to create output file");
    }

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(output.clone())
        .expect("Cannot open file");

    write!(&mut data_file, "# Lunala Patcher\n").expect("Failed to write Header to file");

    if !disabled.is_empty() {
        write!(&mut data_file, "\n## Optional Files\n\n").expect("Failed to write section header to file");
        for file_name in disabled {
            write!(&mut data_file, "{}", format!(" -[_] {}\n", file_name))
                .expect("Failed to write to file, failed to add disabled files");
        }
    }
    if !mods.is_empty() {
        write!(&mut data_file, "\n## Mod Files'\n\n").expect("Failed to write section header to file");
        for file_name in mods {
            write!(&mut data_file, "{}", format!(" -[x] {}\n", file_name))
                .expect("Failed to write to file, failed to add mod files");
        }
    }
}