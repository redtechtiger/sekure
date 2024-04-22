pub fn encrypt(input: String, key: String) -> Result<Vec<u8>, ()> {
    todo!("Encrypting isn't implemented yet");
}

pub fn decrypt(input: Vec<u8>, key: String) -> Result<String, ()> {
    todo!("Decrypting isn't implemented yet");
}

fn quarter_round(a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32) {
    // Stage 1
    *a = a.wrapping_add(*b);
    *d ^= *a;
    *d = d.rotate_left(16);

    // Stage 2
    *c = c.wrapping_add(*d);
    *b ^= *c;
    *b = b.rotate_left(12);

    // Stage 3
    *a = a.wrapping_add(*b);
    *d ^= *a;
    *d = d.rotate_left(8);

    // Stage 4
    *c = c.wrapping_add(*d);
    *b ^= *c;
    *b = b.rotate_left(7);

    // todo!("Operations aren't wrapped yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() { // Test which arithmetic functions we are using
        assert_eq!(0x77777777 as u32 + 0x01234567 as u32, 0x789abcde as u32);
        assert_eq!(0x01020304 as u32 ^ 0x789abcde as u32, 0x7998bfda as u32);
        assert_eq!((0x7998bfda as u32).rotate_left(7) as u32, 0xcc5fed3c as u32);
    }

    #[test]
    fn quarter_round_1() { // Test that the quarter round produces correct results
        let (mut a, mut b, mut c, mut d) = (0x516461b1, 0x2a5f714c, 0x53372767, 0x3d631689);
        quarter_round(&mut a, &mut b, &mut c, &mut d);
        assert_eq!(
            (a, b, c, d),
            (0xbdb886dc, 0xcfacafd2, 0xe46bea80, 0xccc07c79)
        );
    }

    #[test]
    fn quarter_round_2() { // Test that the quarter round produces correct results
        let (mut a, mut b, mut c, mut d) = (0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567);
        quarter_round(&mut a, &mut  b, &mut c, &mut d);
        assert_eq!(
            (a, b, c, d),
            (0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb)
        );
    }
}
