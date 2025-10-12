use clap::Parser;

/// Luna Patcher CLI
#[derive(Parser)]
#[command(name = "luna-patcher")]
#[command(about = "Created versions/patches based on your file changes", version = "1.0")]
pub struct Cli {
    /// The command to execute
    #[arg(short, long)]
    pub command: String,
}

pub fn run(args: Cli) {
    match args.command.as_str() {
        "example" => println!("Running example command"),
        _ => println!("Unknown command: {}", args.command),
    }
}
