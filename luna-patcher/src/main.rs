use clap::Parser;

mod commands;
mod patch;

fn main() {
    let args = commands::Cli::parse();
    println!("Luna Patcher");
    commands::run(args);
}
