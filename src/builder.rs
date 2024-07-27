use crate::chacha20;
use crate::pbkdf2;

pub trait Encrypter {
    fn encrypt(plaintext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
    fn decrypt(ciphertext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
}

pub trait Tagger {
    fn tag(msg: &[u8], key: [u8; 32]) -> [u8; 16];
}

pub trait KeyDeriver {
    fn derive_key(password: &str, salt: [u8; 128]) -> [u8; 32];
}

