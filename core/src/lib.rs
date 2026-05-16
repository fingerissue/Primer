use argon2::{
    password_hash::{ PasswordHasher, SaltString, rand_core::OsRng, },
    Argon2,
};
use zeroize::Zeroizing;
use anyhow::Result;
use serde::{ Serialize, Deserialize };
use std::path::PathBuf;

pub fn hash_password(password: &Zeroizing<String>, salt: &Zeroizing<String>) -> Result<String> {
  let salt_string = SaltString::from_b64(salt.as_str()).map_err(|e| anyhow::anyhow!("Invalid cryptographic salt format: {}", e))?;
  
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt_string).map_err(|e| anyhow::anyhow!("Crypto engine failed to compute password hash: {}", e))?;
  
  Ok(password_hash.to_string())
}

pub fn generate_salt() -> Zeroizing<String> {
  let salt = SaltString::generate(&mut OsRng).to_string();
  Zeroizing::new(salt)
}

pub trait VaultRepository {
    fn save_master_key(&self, hashed_password: &str, salt: &Zeroizing<String>) -> Result<()>;
    fn get_master_key(&self) -> Result<(String, Zeroizing<String>)>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VaultData {
  pub hashed_password: String,
  pub salt: String,
}

pub struct FileStorage {
  pub file_path: PathBuf,
}

impl VaultRepository for FileStorage {
    fn save_master_key(&self, hashed_password: &str, salt: &Zeroizing<String>) -> Result<()> {
        let data = VaultData {
          hashed_password: hashed_password.to_string(),
          salt: salt.as_str().to_string(),
        };
        let json_string = serde_json::to_string_pretty(&data)?;
        std::fs::write(&self.file_path, json_string)?;
        
        Ok(())
    }

    fn get_master_key(&self) -> Result<(String, Zeroizing<String>)> {
        todo!()
    }
}
