pub fn encrypt(input: String, key: [u32; 8], nonce: [u32; 3]) -> Result<Vec<u8>, ()> {
    // Get an initialized ChaCha20 state
    let mut state = init_state(key, nonce);

    for i in 0..11 {
        quarter_round(&mut state, 0, 4, 8, 12);
    }
    
    todo!("Encrypting isn't implemented yet");
}

pub fn decrypt(input: Vec<u8>, key: String) -> Result<String, ()> {
    todo!("Decrypting isn't implemented yet");
}

fn init_state(key: [u32; 8], nonce: [u32; 3]) -> [u32; 16] {
    // Declare matrix (initialization isn't important, it will be overwritten anyways)
    let mut state: [u32; 16] = [0; 16];

    // First row: First 4 constants
    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    // Second row: Key blocks
    state[4] = key[0];
    state[5] = key[1];
    state[6] = key[2];
    state[7] = key[3];

    // Third row: Key blocks continued
    state[8] = key[4];
    state[9] = key[5];
    state[10] = key[6];
    state[11] = key[7];


    // Forth row: Block count (0), and nonce
    state[12] = 0;
    state[13] = nonce[0];
    state[14] = nonce[1];
    state[15] = nonce[2];

    state
}

fn quarter_round(state: &mut [u32; 8], a: usize, b: usize, c: usize, d: usize) {
    // Stage 1
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(16);

    // Stage 2
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(12);

    // Stage 3
    state[a] = state[a].wrapping_add(state[b]);
    state[d] ^= state[a];
    state[d] = state[d].rotate_left(8);

    // Stage 4
    state[c] = state[c].wrapping_add(state[d]);
    state[b] ^= state[c];
    state[b] = state[b].rotate_left(7);
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
    fn quarter_round_1() { // Test that the quarter round produces correct results (2.2.1)
        let mut state = [0x516461b1, 0x2a5f714c, 0x53372767, 0x3d631689, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        quarter_round(&mut a, &mut b, &mut c, &mut d);
        assert_eq!(
            (a, b, c, d),
            (0xbdb886dc, 0xcfacafd2, 0xe46bea80, 0xccc07c79)
        );
    }

    #[test]
    fn quarter_round_2() { // Test that the quarter round produces correct results (2.1.1)
        let (mut a, mut b, mut c, mut d) = (0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567);
        quarter_round(&mut a, &mut  b, &mut c, &mut d);
        assert_eq!(
            (a, b, c, d),
            (0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb)
        );
    }
}
