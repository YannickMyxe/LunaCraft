use clap::Parser;

mod commands;
mod init;

fn main() {
    let args = commands::Cli::parse();
    println!("Luna Patcher");
    let _ = commands::run(args);
}
