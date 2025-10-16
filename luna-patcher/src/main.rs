use clap::Parser;

mod commands;
mod init;
mod config;
mod patch;

fn main() {
    let args = commands::Cli::parse();
    let _ = commands::run(args, "./lunaconfig.toml");
}
