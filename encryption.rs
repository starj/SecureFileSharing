use std::fs::{self, File};
use std::io::{Read, Write};
use std::env;
use ring::aead;
use ring::pbkdf2;
use std::num::NonZeroU32;

mod encryption {
    use super::*;

    const NONCE: &[u8; 12] = b"unique nonce";
    const SALT: &[u8; 16] = b"unique salt here!";
    const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

    fn load_encryption_key() -> ring::aead::LessSafeKey {
        dotenv::dotenv().ok();
        let secret = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
        let mut pbkdf2_hash = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            ITERATIONS,
            SALT,
            secret.as_bytes(),
            &mut pbkdf2_hash,
        );
        let s_key = aead::UnboundKey::new(&aead::AES_256_GCM, &pbkdf2_hash).unwrap();
        aead::LessSafeKey::new(s_key)
    }

    pub fn encrypt(data: &[u8]) -> Vec<u8> {
        let key = load_encryption_key();
        let nonce = aead::Nonce::assume_unique_for_key(*NONCE);
        let mut in_out = data.to_vec();
        let suffix_space = key.algorithm().tag_len();
        in_out.extend(std::iter::repeat(0).take(suffix_space));
        key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .expect("encryption failure!");
        in_out
    }

    pub fn decrypt(encrypted_data: &[u8]) -> Vec<u8> {
        let key = load_encryption_key();
        let nonce = aead::Nonce::assume_unique_for_key(*NONCE);
        let mut in_out = encrypted_data.to_vec();
        key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .expect("decryption failure!")
            .to_vec()
    }

    pub fn encrypt_file(path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let encrypted_data = encrypt(&buffer);
        fs::write(path, encrypted_data)?;
        Ok(())
    }

    pub fn decrypt_file(path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decrypted_data = decrypt(&buffer);
        fs::write(path, decrypted_data)?;
        Ok(())
    }
}