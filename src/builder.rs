use crate::chacha20;
use crate::pbkdf2;

/// Trait for encrypting and decrypting data
pub trait Encrypter {
    fn encrypt(plaintext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
    fn decrypt(ciphertext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
}

/// Trait for tagging data with associated data
pub trait Tagger {
    fn tag(msg: &[u8], key: [u8; 32]) -> [u8; 16];
}

/// Trait for deriving a cryptographic key from a password
pub trait KeyDeriver {
    fn derive_key(password: &str, salt: [u8; 128]) -> [u8; 32];
}
