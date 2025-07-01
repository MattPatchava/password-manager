use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm,
};
use anyhow::{anyhow, Result};
use base64::engine::{general_purpose, Engine};
use rand::{rngs::OsRng, RngCore};
use typenum;

pub fn encrypt_password(cipher: Aes256Gcm, plaintext: &str) -> Result<(Vec<u8>, [u8; 12])> {
    let mut nonce_bytes: [u8; 12] = [0u8; 12];
    let mut rng: OsRng = OsRng::default();
    rng.fill_bytes(&mut nonce_bytes);

    let nonce: &GenericArray<u8, typenum::U12> = GenericArray::from_slice(&nonce_bytes);

    let ciphertext: Vec<u8> = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow!(e))?;

    Ok((ciphertext, nonce_bytes))
}

pub fn decrypt_password(cipher: &Aes256Gcm, nonce: &str, ciphertext: &str) -> Result<String> {
    let nonce_bytes = general_purpose::STANDARD.decode(nonce)?;
    let nonce: &GenericArray<u8, typenum::U12> = GenericArray::from_slice(&nonce_bytes);
    let ciphertext: Vec<u8> = general_purpose::STANDARD.decode(ciphertext)?;

    let plaintext: Vec<u8> = cipher
        .decrypt(nonce, ciphertext.as_slice())
        .map_err(|e| anyhow!(e))?;

    let plaintext_str: String = String::from_utf8(plaintext)?;

    Ok(plaintext_str)
}
