const DIGEST_SIZE: usize = 256; // SHA256 is used

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize>(
    password: &str,
    salt: [u8; 128],
    iteration_count: usize,
) -> [u8; KEYLEN]
where
    [(); derive_len(KEYLEN)]:,
{
    if KEYLEN > (2usize.pow(32) - 1) * DIGEST_SIZE {
        panic!("master key length exceeds maximum allowed by sha256");
    }

    let r = KEYLEN - (derive_len(KEYLEN) - 1) * DIGEST_SIZE;
    let t = [0u8; derive_len(KEYLEN)];

    for i in 1..=derive_len(KEYLEN) {

    }

    todo!();
}

const fn derive_len(keylen: usize) -> usize {
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
