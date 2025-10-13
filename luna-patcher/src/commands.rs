use clap::Parser;

/// Luna Patcher CLI
#[derive(Parser)]
#[command(name = "luna-patcher")]
#[command(about = "Created versions/patches based on your file changes", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// An example command
    Example,
}

pub fn run(cli: Cli) {
    match &cli.command {
        Commands::Example => println!("Running example command"),
    };
}
