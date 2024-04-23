pub fn encrypt(plaintext: String, key: [u32; 8], nonce: [u32; 3], counter: u32) -> Result<Vec<u8>, ()> {
    let mut encrypted_message = 
    // Loop for every 64 characters, i.e. every 512 bits
    for j in 0..(plaintext.len()/64)-1 { // TODO: Check if this works or whether we need floor
        let key_stream = block(key, nonce, counter+j as u32);
        let input_block = &plaintext[j*64..plaintext.len()-1];
    };

    todo!("Encrypting isn't implemented yet");
}

pub fn block(key: [u32; 8], nonce: [u32; 3], counter: u32) -> [u8; 64] {
    // Get initial state
    let init_state = init_state(key, nonce, counter);
    let mut working_state = init_state;

    // Execute rounds
    for _i in 0..10 {
        quarter_round(&mut working_state, 0, 4, 8, 12);
        quarter_round(&mut working_state, 1, 5, 9, 13);
        quarter_round(&mut working_state, 2, 6, 10, 14);
        quarter_round(&mut working_state, 3, 7, 11, 15);
        quarter_round(&mut working_state, 0, 5, 10, 15);
        quarter_round(&mut working_state, 1, 6, 11, 12);
        quarter_round(&mut working_state, 2, 7, 8, 13);
        quarter_round(&mut working_state, 3, 4, 9, 14);
    };
    let final_state = add_states(init_state, working_state);

    // Serialize
    let mut serialized: [u8; 64] = [0; 64];
    for i in 0..16 {
        serialized[4*i..][..4].copy_from_slice(&final_state[i].to_le_bytes());
    }
    serialized
}

pub fn decrypt(input: Vec<u8>, key: String) -> Result<String, ()> {
    todo!("Decrypting isn't implemented yet");
}

fn init_state(key: [u32; 8], nonce: [u32; 3], counter: u32) -> [u32; 16] {
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

    // Forth row: Block count and nonce
    state[12] = counter;
    state[13] = nonce[0];
    state[14] = nonce[1];
    state[15] = nonce[2];

    state
}

fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
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

fn add_states(state1: [u32; 16], state2: [u32; 16]) -> [u32; 16] {
    // Calculate the sum of both matrices
    let mut sum_state: [u32; 16] = [0; 16];
    for i in 0..16 {
        sum_state[i] = state1[i].wrapping_add(state2[i]);
    }
    sum_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        // Test which arithmetic functions we are using
        assert_eq!(0x77777777 as u32 + 0x01234567 as u32, 0x789abcde as u32);
        assert_eq!(0x01020304 as u32 ^ 0x789abcde as u32, 0x7998bfda as u32);
        assert_eq!((0x7998bfda as u32).rotate_left(7) as u32, 0xcc5fed3c as u32);
    }

    #[test]
    fn quarter_round_1() {
        // Test that the quarter round produces correct results (2.2.1)
        let mut state = [
            0x516461b1, 0x2a5f714c, 0x53372767, 0x3d631689, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        quarter_round(&mut state, 0, 1, 2, 3);
        assert_eq!(
            state,
            [0xbdb886dc, 0xcfacafd2, 0xe46bea80, 0xccc07c79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn quarter_round_2() {
        // Test that the quarter round produces correct results (2.2.1)
        let mut state = [
            0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        quarter_round(&mut state, 0, 1, 2, 3);
        assert_eq!(
            state,
            [0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn init_state_1() {
        // Test that the state initializes correctly
        let state = init_state([0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c], [0x09000000, 0x4a000000, 0x00000000], 1);
        assert_eq!(
            state,
            [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000, 0x4a000000, 0x00000000]
            );
    }

    #[test]
    fn add_states_1() {
        let state1 = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000, 0x4a000000, 0x00000000];
        let state2 = [0x837778abu32, 0xe238d763u32, 0xa67ae21e, 0x5950bb2f, 0xc4f2d0c7, 0xfc62bb2f, 0x8fa018fc, 0x3f5ec7b7, 0x335271c2, 0xf29489f3, 0xeabda8fc, 0x82e46ebd, 0xd19c12b4, 0xb04e16de, 0x9e83d0cb, 0x4e3c50a2];
        assert_eq!(
            add_states(state1, state2),
            [0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2]
            );
    }

    #[test]
    fn block_1() {
        let out = block([0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c], [0x09000000, 0x4a000000, 0x00000000], 1);
        assert_eq!(
            out,
            [0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20, 0x71, 0xc4, 0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a, 0xc3, 0xd4, 0x6c, 0x4e, 0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09, 0x14, 0xc2, 0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2, 0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9, 0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e]

        )
    }

}

