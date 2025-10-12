use clap::Parser;

mod commands;

fn main() {
    let args = commands::Cli::parse();
    println!("Luna Patcher");
    commands::run(args);
}
