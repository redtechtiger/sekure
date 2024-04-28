pub fn generate(msg: &[u8], key: [u8; 32]) -> Vec<u8> {
    let r = clamp(&mut key[0..15].try_into().unwrap()); // TODO: Figure out if we can get rid of
                                                        // the ugly .try_into().unwrap()
    let s = &key[16..31];
    let P = 0x3fffffffffffffffffffffffffffffffbu128; // TODO: Fix this garbage
    let mut acc = 0;

    for i in 0..msg.len().div_ceil(16) { // Iterate over every 16 byte block
         // Do stuff (see reference RFC 7539, 2.5)
    }

    todo!();
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
        assert_eq!(
            clamp([0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33, 0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5, 0x06, 0xa8]),
            []
            )
    }
}

