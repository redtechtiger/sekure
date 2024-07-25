use hmac::{Hmac, Mac};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use sha2::Sha256;
const DIGEST_SIZE: usize = 256; // SHA256 is used
type DigestType = [u8; DIGEST_SIZE / 8];

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize, const ITERATION_COUNT: usize>(
    password: &str,
    salt: [u8; 128],
) -> [u8; KEYLEN / 8]
where
    [(); derive_num_blocks(KEYLEN)]:,
    [(); ITERATION_COUNT + 1]:,
{
    // Check that values are valid
    if KEYLEN > (2usize.pow(32) - 1) * DIGEST_SIZE {
        panic!("derived key too long");
    }

    // Derive the actual cryptographic key
    let mut t: [DigestType; derive_num_blocks(KEYLEN)] =
        [[0u8; DIGEST_SIZE / 8]; derive_num_blocks(KEYLEN)];
    for i in 0..derive_num_blocks(KEYLEN) {
        t[i] = f::<ITERATION_COUNT>(password, salt, i);
    }

    // Convert to suitable output format TODO: Make this safer: Currently KEYLEN has to be a factor of DIGEST_SIZE
    let mut out = [0u8; KEYLEN / 8];
    for i in 0..t.len() {
        out[i * (DIGEST_SIZE / 8)..i * (DIGEST_SIZE / 8) + (DIGEST_SIZE / 8)]
            .copy_from_slice(&t[i]);
    }

    out
}

fn f<const ITERATION_COUNT: usize>(password: &str, salt: [u8; 128], index: usize) -> DigestType {
    // Initial hash will be salt with the index concatenated onto it
    let mut initial_concat = [0u8; 128 + 4]; // Length of salt plus index (converted to 4 bytes)
    initial_concat[0..128].copy_from_slice(&salt);
    initial_concat[128..].copy_from_slice(&(index as i32).to_be_bytes());

    let mut u = hmac_sha256(password, &initial_concat);
    for _ in 1..ITERATION_COUNT {
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

fn hmac_sha256(password: &str, input: &[u8]) -> [u8; DIGEST_SIZE / 8] {
    let mut mac = Hmac::<Sha256>::new_from_slice(password.as_bytes())
        .expect("hmac key initialization failed");
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
        assert_eq!(
            derive_cryptographic_key::<512, 1_000>("foobar1", salt),
            [
                232, 255, 43, 25, 232, 12, 184, 3, 14, 126, 118, 158, 103, 110, 93, 154, 99, 199,
                212, 112, 141, 64, 28, 152, 125, 70, 93, 187, 31, 252, 74, 87, 246, 244, 56, 48,
                180, 45, 137, 109, 214, 196, 254, 196, 163, 95, 129, 243, 37, 25, 253, 248, 20, 53,
                238, 13, 133, 91, 223, 36, 91, 44, 97, 154,
            ]
        );
    }
    #[test]
    fn derive_cryptographic_key_2() {
        // Test to check that a very high iteration count doesn't tank performance

        // "Random" salt that stays constant between tests
        let mut rng = ChaCha8Rng::seed_from_u64(17);
        let mut salt: [u8; 128] = [0; 128];
        for i in 0..128 {
            salt[i] = rng.gen_range(0..=255);
        }

        assert_eq!(
            derive_cryptographic_key::<512, 100_010>("my_password_8", salt),
            [
                74, 237, 188, 63, 30, 178, 153, 229, 105, 180, 209, 208, 185, 205, 19, 55, 254,
                115, 208, 100, 219, 24, 146, 21, 231, 95, 131, 137, 244, 165, 216, 237, 28, 20, 64,
                78, 214, 74, 50, 110, 236, 1, 58, 248, 218, 47, 106, 87, 130, 236, 193, 21, 199,
                45, 105, 18, 132, 15, 28, 234, 14, 179, 52, 93
            ]
        );
    }
}
