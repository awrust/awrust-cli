use clap::{Parser, Subcommand};

mod client;
mod cmd;
mod error;

#[derive(Parser)]
#[command(name = "awr")]
struct Cli {
    #[arg(long, env = "AWRUST_ENDPOINT", default_value = "http://localhost:4566")]
    endpoint: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    S3 {
        #[command(subcommand)]
        action: cmd::s3::S3Command,
    },
    Status,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = client::Client::new(&cli.endpoint);
    let result = match cli.command {
        Command::S3 { action } => cmd::s3::execute(&client, action).await,
        Command::Status => cmd::status::execute(&client).await,
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
