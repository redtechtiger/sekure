use crate::bignum::BigU288;

pub fn generate(msg: &[u8], key: [u8; 32]) -> Vec<u8> {
    let mut r: [u8; 16] = key[0..16].try_into().unwrap();
    clamp(&mut r); // TODO: Figure out if we can get rid of
                   // the ugly .try_into().unwrap()
    let r = BigU288::from_slice(&r);
    let s = &key[16..31];
    let p = BigU288::from_hex("3fffffffffffffffffffffffffffffffb"); // TODO: Fix this garbage
    let mut acc = BigU288::new();

    for i in 0..msg.len().div_ceil(16) {
        dbg!(i);
        // Iterate over every 16 byte block
        let mut n: BigU288 = BigU288::from_slice(&msg[i * 16..i * 16 + 15]);
        dbg!(n.get_bytes());
        // n.add_msb(); // Find a way of fixing this!
        acc = acc + n;
        dbg!(r.get_bytes());
        dbg!(acc*r);
        dbg!((acc*r)%p);
        acc = (acc * r) % p;
        dbg!(acc.get_bytes());
    }

    acc = acc + BigU288::from_slice(s);
    // todo!();
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

    #[test]
    fn generate_1() {
        generate(
            b"Cryptographic Forum Research Group",
            [
                0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5,
                0x06, 0xa8, 0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd, 0x4a, 0xbf, 0xf6, 0xaf,
                0x41, 0x49, 0xf5, 0x1b,
            ],
        );
        todo!("test isn't done, nor is the implementation, panic so that we can see debug log");
    }
}
