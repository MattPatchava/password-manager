use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm,
};
use anyhow::{anyhow, Result};
use base64::engine::{general_purpose, Engine};
use rand::{rngs::OsRng, RngCore};
use typenum;

pub fn encrypt_password(cipher: Aes256Gcm, plaintext: &str) -> Result<(String, String)> {
    let mut nonce_bytes: [u8; 12] = [0u8; 12];
    let mut rng: OsRng = OsRng::default();
    rng.fill_bytes(&mut nonce_bytes);

    let nonce: &GenericArray<u8, typenum::U12> = GenericArray::from_slice(&nonce_bytes);

    let ciphertext: Vec<u8> = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow!(e))?;

    Ok((
        general_purpose::STANDARD.encode(ciphertext),
        general_purpose::STANDARD.encode(nonce),
    ))
}
