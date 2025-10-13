
pub fn init(files: Vec<String>, output: Option<String>) {
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

fn print(disabled: &Vec<String>, mods: &Vec<String>, output: Option<String>) {
    
    let file = std::fs::File::create(output.unwrap_or("changes.md".to_string()));
    println!("Output file: {:?}", file);

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
