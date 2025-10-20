use clap::Parser;

mod commands;
mod init;
mod config;
mod patch;
mod files;

fn main() {
    let args = commands::Cli::parse();
    let result = commands::run(args, "./lunaconfig.toml");
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
