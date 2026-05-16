use primer_core::{ generate_salt, hash_password, FileStorage, VaultRepository };
use clap::{ Parser, Subcommand };
use anyhow::Result;
use zeroize::Zeroizing;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "primer")]
#[command(about = "A production-ready CLI password manager", long_about = None)]
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
    Commands::Init => handle_init()?,
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

fn handle_init() -> Result<()> {
  println!("Initializing the vault...");
  
  let password = Zeroizing::new(rpassword::prompt_password("Enter master password: ")?);
  let password_confirm = Zeroizing::new(rpassword::prompt_password("Confirm master password: ")?);
  let is_match: bool = subtle::ConstantTimeEq::ct_eq(password.as_bytes(), password_confirm.as_bytes()).into();
  if !is_match {
    return Err(anyhow::anyhow!("Passwords do not match. Verification failed."));
  }
  
  println!("Master password set successfully.");
  
  let salt = generate_salt();
  
  let hash = hash_password(&password, &salt)?;
  let save_path = PathBuf::from("vault.json");
  let storage = FileStorage {
    file_path: save_path.clone(),
  };
  storage.save_master_key(&hash, &salt)?;
  
  println!("Vault initialized successfully at {}", save_path.display());
  
  Ok(())
}