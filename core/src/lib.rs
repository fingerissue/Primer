use argon2::{
    password_hash::{
        PasswordHasher, SaltString,
    },
    Argon2,
};
use zeroize::Zeroize;

pub fn hash_password(mut password: String, salt: &str) -> Result<String, argon2::password_hash::Error> {
  let salt_obj = SaltString::from_b64(salt)?;
  let argon2 = Argon2::default();
  
  let password_hash = argon2.hash_password(password.as_bytes(), &salt_obj);
  password.zeroize();
  
  match password_hash {
    Ok(hash) => Ok(hash.to_string()),
    Err(e) => Err(e)
  }
}