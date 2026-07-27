//! Main CLI entry point for `env-vault` / `cli-secrets`.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "env-vault")]
#[command(bin_name = "env-vault")]
#[command(author = "un-earthly")]
#[command(version = "0.1.0")]
#[command(about = "High-performance, zero-knowledge CLI to back up and sync local env files and SSH configs.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive token or Device Flow login to backend server
    Login {
        /// Backend API URL
        #[arg(long, default_value = "http://localhost:8000")]
        server: String,
    },
    /// Scan, encrypt, and push local configurations to the cloud
    Push {
        /// Project identifier
        #[arg(short, long)]
        project: String,

        /// Path to target file or directory (defaults to current directory env files)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Pull, decrypt, and restore local configurations from the cloud
    Pull {
        /// Project identifier
        #[arg(short, long)]
        project: String,

        /// Destination path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login { server } => {
            println!("Logging in to server: {}", server);
            // Scaffolding: login code goes here
        }
        Commands::Push { project, path } => {
            println!(
                "Pushing configurations for project: '{}' (path: {:?})",
                project,
                path.as_ref().unwrap_or(&PathBuf::from("."))
            );
            // Scaffolding: scanning, encryption, and API call go here
        }
        Commands::Pull { project, output } => {
            println!(
                "Pulling configurations for project: '{}' to output path: {:?}",
                project,
                output.as_ref().unwrap_or(&PathBuf::from("."))
            );
            // Scaffolding: fetching, decryption, and write-to-disk go here
        }
    }

    Ok(())
}
