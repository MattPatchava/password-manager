use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose};
use rand::RngCore;
use rand::rngs::OsRng;

pub fn generate_salt() -> String {
    let mut salt = [0u8; 32];
    let mut rng: OsRng = OsRng::default();

    rng.fill_bytes(&mut salt);

    general_purpose::STANDARD.encode(salt)
}

pub fn hash_password(master_password: &str, salt: Vec<u8>) -> Result<[u8; 32]> {
    let mut aes_encryption_key: [u8; 32] = [0u8; 32];

    let argon2 = argon2::Argon2::default();

    argon2
        .hash_password_into(&master_password.as_bytes(), &salt, &mut aes_encryption_key)
        .map_err(|e| anyhow!(e))?;

    Ok(aes_encryption_key)
}
