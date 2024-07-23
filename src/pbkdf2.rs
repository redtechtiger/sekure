use sha2::Sha256;
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
    if KEYLEN > (2usize.pow(32) - 1) * DIGEST_SIZE {
        panic!("derived key too long");
    }

    let octets_in_last_block = KEYLEN - (derive_num_blocks(KEYLEN) - 1) * DIGEST_SIZE; // Needed?
    let mut t: [DigestType; KEYLEN] = [DigestType::new(); KEYLEN];
    for i in 1..=derive_num_blocks(KEYLEN) {
        t[i] = f::<ITERATION_COUNT>(password, salt, i);
    }

    todo!();
}

fn f<const ITERATION_COUNT: usize>(password: &str, salt: [u8; 128], index: usize) -> DigestType {

    todo!();
}

const fn derive_num_blocks(keylen: usize) -> usize {
    (keylen / DIGEST_SIZE) + (keylen % DIGEST_SIZE != 0) as usize
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
