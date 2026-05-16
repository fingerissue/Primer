use argon2::{
    password_hash::{ PasswordHasher, SaltString, rand_core::OsRng, },
    Argon2,
};
use zeroize::Zeroizing;

pub fn hash_password(password: &Zeroizing<String>, salt: &str) -> Result<String, argon2::password_hash::Error> {
  let salt_obj = SaltString::from_b64(salt)?;
  let argon2 = Argon2::default();
  
  let password_hash = argon2.hash_password(password.as_bytes(), &salt_obj)?;
  
  Ok(password_hash.to_string())
}

pub fn generate_salt() -> Zeroizing<String> {
  let salt = SaltString::generate(&mut OsRng).to_string();
  Zeroizing::new(salt)
}