use clap::Parser;

mod commands;
mod init;

fn main() {
    let args = commands::Cli::parse();
    let _ = commands::run(args, "./lunaconfig.toml");
}
