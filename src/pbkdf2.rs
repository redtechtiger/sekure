const DIGEST_SIZE: usize = 256; // SHA256 is used

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize>(
    password: &str,
    salt: [u8; 128],
    iteration_count: usize,
) -> [u8; KEYLEN] {
    if KEYLEN > (2usize.pow(32)-1) * DIGEST_SIZE {
        panic!("master key length exceeds maximum allowed by sha256");
    }

    const len: usize = (KEYLEN / DIGEST_SIZE) + (KEYLEN % DIGEST_SIZE != 0) ;

    let r = KEYLEN - (len - 1) * 256;
    let t = [0u8; len];

    for i in 1..=len {
        
    }
    
    todo!();
}
