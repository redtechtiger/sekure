use sha2::Sha256;
use hmac::{Hmac, Mac};
use rand_chacha::ChaCha8Rng;
use rand::prelude::*;
const DIGEST_SIZE: usize = 256; // SHA256 is used
type DigestType = [u8; DIGEST_SIZE / 8];

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize, const ITERATION_COUNT: usize>(
    password: &str,
    salt: [u8; 128],
) -> [u8; KEYLEN/8]
where
    [(); derive_num_blocks(KEYLEN)]:,
    [(); ITERATION_COUNT+1]:,
{
    // Check that values are valid
    if KEYLEN > (2usize.pow(32) - 1) * DIGEST_SIZE {
        panic!("derived key too long");
    }

    // Derive the actual cryptographic key
    let mut t: [DigestType; derive_num_blocks(KEYLEN)] = [[0u8; DIGEST_SIZE / 8]; derive_num_blocks(KEYLEN)];
    for i in 0..derive_num_blocks(KEYLEN) {
        t[i] = f::<ITERATION_COUNT>(password, salt, i);
    }

    dbg!(t);
    
    // Convert to suitable output format TODO: Make this safer: Currently KEYLEN has to be a factor of DIGEST_SIZE
    let mut out = [0u8; KEYLEN/8];
    for i in 0..t.len() {
        dbg!(t.len());
        out[i*(DIGEST_SIZE/8)..i*(DIGEST_SIZE/8)+(DIGEST_SIZE/8)].copy_from_slice(&t[i]);
    }

    out
}

fn f<const ITERATION_COUNT: usize>(password: &str, salt: [u8; 128], index: usize) -> DigestType {
    // Initial hash will be salt with the index concatenated onto it
    let mut initial_concat = [0u8; 128+4]; // Length of salt plus index (converted to 4 bytes)
    initial_concat[0..128].copy_from_slice(&salt);
    initial_concat[128..].copy_from_slice(&(index as i32).to_be_bytes());

    let mut u = hmac_sha256(password, &initial_concat);
    for _ in 1..ITERATION_COUNT {
        // dbg!(u);
        u = xor_digest_type(u, hmac_sha256(password, &u));
    }
    u
}

const fn derive_num_blocks(keylen: usize) -> usize {
    (keylen / DIGEST_SIZE) + (keylen % DIGEST_SIZE != 0) as usize
}

fn xor_digest_type(a: DigestType, b: DigestType) -> DigestType {
    let mut out = [0u8; DIGEST_SIZE / 8];
    for i in 0..a.len() {
        out[i] = a[i] ^ b[i];
    }
    out
}

fn hmac_sha256(password: &str, input: &[u8]) -> [u8; DIGEST_SIZE/8] {
    let mut mac = Hmac::<Sha256>::new_from_slice(password.as_bytes()).expect("hmac key initialization failed");
    mac.update(input);
    let result = mac.finalize();
    result.into_bytes().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Temporary test case to observe function behavior
    #[test]
    fn derive_cryptographic_key_1() {
        // "Random" salt that stays constant between tests
        let mut rng = ChaCha8Rng::seed_from_u64(17);
        let mut salt: [u8; 128] = [0; 128];
        for i in 0..128 {
            salt[i] = rng.gen_range(0..=255);
        }
        dbg!(salt);
        dbg!(derive_cryptographic_key::<512, 1000>("foobar", salt));
    }
}
