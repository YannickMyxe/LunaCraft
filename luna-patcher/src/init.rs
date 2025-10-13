use std::{fs, io::Write, os::windows::io::{AsHandle, AsRawHandle}};

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
    let _ = std::fs::File::create(output.clone()).expect("Failed to create output file");
    fs::write(output.clone(), "# Lunala Patcher \n").expect("Failed to write Header to file");

    if !disabled.is_empty() {
        for file_name in disabled {
            fs::write(&output, format!(" -[_] {}\n", file_name))
                .expect("Failed to write to file, failed to add disabled files");
        }
    }
    if !mods.is_empty() {
        for file_name in mods {
                       fs::write(&output, format!(" -[x] {}\n", file_name))
                            .expect("Failed to write to file, failed to add mod files");
        }
    }
}