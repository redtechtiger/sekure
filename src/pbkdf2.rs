use typenum::{PartialDiv, Integer, P256};
use typenum_consts::uconst;
type DIGEST_SIZE = P256; // SHA256 is used

// KEYLEN is the amount of bytes the cryptographic key shall contain
pub fn derive_cryptographic_key<const KEYLEN: usize>(
    password: &str,
    salt: [u8; 128],
    iteration_count: usize,
) -> [u8; KEYLEN] {
    if KEYLEN > (2usize.pow(32)-1) * <DIGEST_SIZE as Integer>::to_isize() as usize {
        panic!("master key length exceeds maximum allowed by sha256");
    }

    type len = PartialDiv<uconst![KEYLEN], DIGEST_SIZE>;
    const len: usize = <len as Integer>::to_i64() as usize;

    let r = KEYLEN - (len - 1) * 256;
    let t = [0u8; len];

    for i in 1..=len {
        
    }
    
    todo!();
}
