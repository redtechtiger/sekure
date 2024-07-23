use sha2::Sha256;
use hmac::{Hmac, Mac};
use sha2::digest::typenum::U256;
const DIGEST_SIZE: usize = 256; // SHA256 is used
type DigestType = U256;

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize, const ITERATION_COUNT: usize>(
    password: &str,
    salt: [u8; 128],
) -> [u8; KEYLEN]
where
    [(); derive_num_blocks(KEYLEN)]:,
    [(); ITERATION_COUNT+1]:,
{
    // Check that values are valid
    if KEYLEN > (2usize.pow(32) - 1) * DIGEST_SIZE {
        panic!("derived key too long");
    }

    // Derive the actual cryptographic key
    let octets_in_last_block = KEYLEN - (derive_num_blocks(KEYLEN) - 1) * DIGEST_SIZE; // Needed?
    let mut t: [DigestType; KEYLEN] = [DigestType::new(); KEYLEN];
    for i in 1..=derive_num_blocks(KEYLEN) {
        t[i] = f::<ITERATION_COUNT>(password, salt, i);
    }
    
    // Convert to suitable output format
    let mut out = [0u8; KEYLEN];
    for block in t {
        // t[0].
    }

    out
}

fn f<const ITERATION_COUNT: usize>(password: &str, salt: [u8; 128], index: usize) -> DigestType {
    
    todo!();
}

const fn derive_num_blocks(keylen: usize) -> usize {
    (keylen / DIGEST_SIZE) + (keylen % DIGEST_SIZE != 0) as usize
}

fn hmac_sha256(password: &str, input: &[u8]) -> [u8; 256/8] {
    let mut mac = Hmac::<Sha256>::new_from_slice(password.as_bytes()).expect("hmac key initialization failed");
    mac.update(input);
    let result = mac.finalize();
    result.into_bytes().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_cryptographic_key_1() {
        // Temporary test case to observe function behavior
        // derive_cryptographic_key::<512>("foobar", [0; 128], 10_000);
    }
}
