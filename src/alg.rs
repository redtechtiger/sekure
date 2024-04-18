pub fn encrypt(input: String, key: String) -> Result<Vec<u8>, ()> {

    todo!();
}

pub fn decrypt(input: Vec<u8>, key: String) -> Result<String, ()> {

    todo!();
}

fn quarter_round(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
    // Stage 1
    *a += *b;
    *d ^= *a;
    *d <<= 16;

    // Stage 2
    *c += *d;
    *b ^= *c;
    *b <<= 12;

    // Stage 3
    *a += *b;
    *d ^= *a;
    *d <<= 8;

    // Stage 4
    *c += *d;
    *b ^= *c;
    *b <<= 7;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_round_1() {
        let (mut a,mut b,mut c,mut d) = (0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567);
        quarter_round(&mut a, &mut b, &mut c, &mut d);
        assert_eq!((a,b,c,d),(0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb));
    }
}

