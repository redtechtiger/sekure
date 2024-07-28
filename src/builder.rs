use crate::chacha20;
use crate::poly1305;
use crate::pbkdf2;
use rand::random;
use rand::Rng;

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

#[derive(Debug)]
pub struct SekureIO<'a> {
    path: &'a str,
    plaintext_buffer: Vec<u8>,
    master_key: [u8; 32],
    salt: [u8; 128],
}

impl<'a> SekureIO<'a> {
    fn create(path: &'a str, password: &str) -> SekureIO<'a> {
        let salt = generate_salt();
        let master_key = pbkdf2::derive_cryptographic_key::<256, 10_000>(password, salt);
        SekureIO {
            path,
            plaintext_buffer: vec![],
            master_key,
            salt,
        }
    }
    fn write(&mut self, data: &[u8]) {
        self.plaintext_buffer.extend_from_slice(data);
    }
}

fn generate_salt() -> [u8; 128] {
    // Generate a random salt
    let mut salt: [u8; 128] = [0u8; 128];
    let mut rng = rand::thread_rng();
    rng.fill(&mut salt);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_1() {
        // Dummy test to check that sample usage compiles
        let my_file = SekureIO::create("foo.bar", "password");
        dbg!(my_file);
    }
}
