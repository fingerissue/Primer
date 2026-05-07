use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "primer")]
#[command(about = "A production-grade CLI password manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Init,
  Add {
    #[arg(value_name = "SERVICE")]
    service: String,
  },
  Get {
    #[arg(value_name = "SERVICE")]
    service: String,
  },
  List,
  Remove {
    #[arg(value_name = "SERVICE")]
    service: String,
  },
}

fn main() -> Result<()> {
  let cli = Cli::parse();
  match cli.command {
    Commands::Init => {
      println!("Initializing the vault...");
    }
    Commands::Add {service} => {
      println!("Adding entry for: {}", service);
    }
    Commands::Get {service} => {
      println!("Retrieving credentials for: {}", service);
    }
    Commands::List => {
      println!("Listing all services...");
    }
    Commands::Remove {service} => {
      println!("Removing entry: {}", service);
    }
  }
  Ok(())
}