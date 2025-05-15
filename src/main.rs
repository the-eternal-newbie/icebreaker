mod chat;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "icebreaker", version, about = "LAN UDP real-time chat inspired by Neuromancer.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a chat listener
    Listen {
        #[arg(short, long)]
        port: u16,
    },
    /// Connect and start sending messages
    Send {
        #[arg(short, long)]
        target: String,
        #[arg(short, long)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Listen { port } => {
            chat::listen(port).await?;
        }
        Commands::Send { target, port } => {
            chat::send_loop(&target, port).await?;
        }
    }

    Ok(())
}
