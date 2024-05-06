use crate::bignum::BigU288;

pub fn generate(msg: &[u8], key: [u8; 32]) -> Vec<u8> {
    let mut r = key[0..15].try_into().unwrap();
    clamp(&mut r); // TODO: Figure out if we can get rid of
                   // the ugly .try_into().unwrap()
    let s = &key[16..31];
    let P = BigU288::from_hex("3fffffffffffffffffffffffffffffffb"); // TODO: Fix this garbage
    let mut acc = BigU288::new();

    for i in 0..msg.len().div_ceil(16) {
        // Iterate over every 16 byte block
        // Do stuff (see reference RFC 7539, 2.5)
        let mut n: BigU288 = BigU288::from_slice(&msg[i * 16..i * 16 + 15]);
        n.add_msb();
        acc = acc + n;
        acc = (acc * r) % P;
    }

    acc = acc + BigU288::from_slice(s);
    todo!();
    acc.get_bytes().to_vec()
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
}
