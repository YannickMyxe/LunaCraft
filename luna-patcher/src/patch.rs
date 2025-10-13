pub fn patch(files: Vec<String>) {
    println!("Patching files...");
    for file in files {
        println!("File: {}", file);
        match file.split(".").last() {
            Some("disabled") => println!(" - This file is disabled"),
            Some(_) => println!(" - This is a regular file"),
            None => println!(" - Unknown file type"),
        }
    }
}
