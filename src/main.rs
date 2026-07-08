use clap::{Args, Parser, Subcommand};

#[derive(Args)]
struct HelloArgs {
    #[arg(long)]
    name: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print version")]
    Version,

    #[command(about = "Say hello")]
    Hello(HelloArgs),
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Version => {
            println!("v{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Hello(args) => {
            println!("Hello {}!", args.name);
        }
    }
}
