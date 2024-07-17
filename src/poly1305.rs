use super::chacha20::block;
use crypto_bigint::{Encoding, NonZero, U256};

pub fn generate_tag(msg: &[u8], key: [u8; 32]) -> [u8; 16] {
    let mut r: [u8; 16] = key[0..16].try_into().unwrap();
    clamp(&mut r);
    let p = NonZero::new(U256::from_be_hex(
        "00000000000000000000000000000003fffffffffffffffffffffffffffffffb",
    ))
    .unwrap(); // Large prime constant
    let r = U256::from(u128::from_le_bytes(r));
    let s: [u8; 16] = key[16..32].try_into().unwrap();
    let mut acc = U256::ZERO;

    for i in 0..msg.len().div_ceil(16) {
        let bytes_read = std::cmp::min(msg.len() - i * 16, 16) as usize;
        // We need to copy a 16 byte chunk of data (or less!) into the U256 'n'.
        let mut n: [u8; 16] = [0; 16]; // Zero-initialize
        n[..bytes_read].copy_from_slice(&msg[i * 16..std::cmp::min(i * 16 + 16, msg.len())]);
        let mut n: U256 = U256::from(u128::from_le_bytes(n));
        // Add one bit beyond the number of bytes read
        // I.e., 1 byte  -> add 0000 0001 0000
        //       2 bytes -> add 0001 0000 0000
        let mut add_msb = [0u8; 32]; // Biggest number we'll ever add is the 17th byte since we're
                                     // reading 16 bytes

        // This is still slow - TODO: Optimize this
        for i in 0..17 {
            add_msb[i] = (bytes_read == i) as u8;
        }

        n = n.wrapping_add(&U256::from_le_slice(&add_msb));

        // Fancy 1305 math
        acc = acc.wrapping_add(&n);
        acc = acc.wrapping_mul(&r);
        acc = acc.div_rem(&p).1; // Select remainder
    }

    acc = acc.wrapping_add(&U256::from(u128::from_le_bytes(s)));
    acc.to_le_bytes()[0..16].try_into().unwrap()
}

pub fn generate_poly_key(key: [u32; 8], nonce: [u32; 3]) -> [u8; 32] {
    let block = block(key, nonce, 0);
    let kept_state: [u32; 8] = block[0..8].try_into().unwrap();

    // Serialize this! 8 * u32 = 32 * u8
    let mut serialized_key = [0u8; 32];
    for i in 0..kept_state.len() {
        serialized_key[i * 4..][..4].copy_from_slice(&kept_state[i].to_le_bytes());
    }
    serialized_key
}

fn clamp(r: &mut [u8; 16]) -> () {
    // Clear top 4 bits
    r[3] &= 0b00001111;
    r[7] &= 0b00001111;
    r[11] &= 0b00001111;
    r[15] &= 0b00001111;

    // Clear bottom two bits
    r[4] &= 0b11111100;
    r[8] &= 0b11111100;
    r[12] &= 0b11111100;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_1() {
        let mut r = [
            0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5,
            0x06, 0xa8,
        ];
        clamp(&mut r);
        assert_eq!(
            r,
            [
                0b10000101, 0b11010110, 0b10111110, 0b00001000, 0b01010100, 0b01010101, 0b01101101,
                0b00000011, 0b01111100, 0b01000100, 0b01010010, 0b00001110, 0b01000000, 0b11010101,
                0b00000110, 0b00001000
            ]
        )
    }

    #[test]
    fn generate_tag_1() {
        assert_eq!(
            generate_tag(
                b"Cryptographic Forum Research Group",
                [
                    0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42,
                    0xd5, 0x06, 0xa8, 0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf,
                    0xf6, 0xaf, 0x41, 0x49, 0xf5, 0x1b,
                ],
            ),
            [
                0xa8, 0x06, 0x1d, 0xc1, 0x30, 0x51, 0x36, 0xc6, 0xc2, 0x2b, 0x8b, 0xaf, 0x0c, 0x01,
                0x27, 0xa9
            ]
        )
    }

    #[test]
    fn generate_poly_key_1() {
        assert_eq!(
            generate_poly_key([0x83828180, 0x87868584, 0x8b8a8988, 0x8f8e8d8c, 0x93929190, 0x97969594, 0x9b9a9998, 0x9f9e9d9c], [0x00000000, 0x03020100, 0x07060504]),
            [0x8a, 0xd5, 0xa0, 0x8b, 0x90, 0x5f, 0x81, 0xcc, 0x81, 0x50, 0x40, 0x27, 0x4a, 0xb2, 0x94, 0x71, 0xa8, 0x33, 0xb6, 0x37, 0xe3, 0xfd, 0x0d, 0xa5, 0x08, 0xdb, 0xb8, 0xe2, 0xfd, 0xd1, 0xa6, 0x46]
        );
    }
}
