use crate::chacha20;
use crate::pbkdf2;

// /// Trait for encrypting and decrypting data
// pub trait Encrypter {
//     fn encrypt(plaintext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
//     fn decrypt(ciphertext: &[u8], key: [u32; 8], nonce: [u32; 3]) -> Vec<u8>;
// }
// 
// /// Trait for tagging data with associated data
// pub trait Tagger {
//     fn tag(msg: &[u8], key: [u8; 32]) -> [u8; 16];
// }
// 
// /// Trait for deriving a cryptographic key from a password
// pub trait KeyDeriver {
//     fn derive_key(password: &str, salt: [u8; 128]) -> [u8; 32];
// }
// 
// /// Main struct for interfacing with encrypted files
// pub struct SekureIO<'a, A: Encrypter, B: Tagger, C: KeyDeriver> {
//     sekure_config: SekureConfig<'a, A, B, C>,
//     plaintext_buffer: Vec<u8>,
// }
// 
// pub struct SekureConfig<'a, A: Encrypter, B: Tagger, C: KeyDeriver> {
//     encrypter: A,
//     tagger: B,
//     key_deriver: C,
//     path: &'a str,
// }
// 
// impl<'a, A: Encrypter, B: Tagger, C: KeyDeriver> SekureConfig<'a, A, B, C> {
//     fn default() -> Self{
//         SekureConfig {
//             encrypter:
//         }
//     }
// }
// 
// impl<'a, A: Encrypter, B: Tagger, C: KeyDeriver> SekureIO<'a, A, B, C> {
//     fn build(encrypter: A, tagger: B, key_deriver: C) {
//         
//     }
// }
