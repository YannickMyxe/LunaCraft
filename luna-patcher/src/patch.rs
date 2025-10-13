pub fn patch(files: Vec<String>) {
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

    print(&disabled, &mods);
}

fn print(disabled: &Vec<String>, mods: &Vec<String>) {
    if !disabled.is_empty() {
        println!("Disabled files");
        println!("==============");
        for file in disabled {
            println!(" -[_] {}", file);
        }
    }
    if !mods.is_empty() {
        println!("Mod files");
        println!("=========");
        for file in mods {
            println!(" -[x] {}", file);
        }
    }
}
