pub fn encrypt(plaintext: &[u8], key: [u32; 8], nonce: [u32; 3], counter: u32) -> Vec<u8> {
    let mut encrypted_message: Vec<u8> = Vec::new();
    // Loop for every 64 characters, i.e. every 512 bits
    for i in 0..(plaintext.len() / 64) {
        // TODO: Check if this works or whether we need floor
        let key_stream = block(key, nonce, counter + i as u32);
        let input_block: &[u8] = &plaintext[i * 64..i * 64 + 64]; // Grab the current block of 64 characters/512 bits, and serialize into bytes.
        let encrypted_block = xor_serialized(&serialize_state(key_stream), &input_block);
        encrypted_message.extend_from_slice(&encrypted_block);
    }
    // Check if there's a partial block left
    if plaintext.len() % 64 != 0 {
        let i = plaintext.len() / 64;
        let key_stream = block(key, nonce, counter + i as u32);
        let input_block: &[u8] = &plaintext[i * 64..plaintext.len()];
        let encrypted_block = xor_serialized(&serialize_state(key_stream), &input_block);
        encrypted_message.extend_from_slice(&encrypted_block[0..plaintext.len() % 64]);
    }
    encrypted_message
}

pub fn decrypt(encrypted: &[u8], key: [u32; 8], nonce: [u32; 3], counter: u32) -> Vec<u8> {
    let mut plaintext: Vec<u8> = Vec::new();
    // Loop for every 64 characters, i.e. every 512 bits
    for i in 0..(encrypted.len() / 64) {
        // TODO: Check if this works or whether we need floor
        let key_stream = block(key, nonce, counter + i as u32);
        let input_block: &[u8] = &encrypted[i * 64..i * 64 + 64]; // Grab the current block of 64 characters/512 bits, and serialize into bytes.
        let encrypted_block = xor_serialized(&serialize_state(key_stream), &input_block);
        plaintext.extend_from_slice(&encrypted_block);
    }
    // Check if there's a partial block left
    if encrypted.len() % 64 != 0 {
        let i = encrypted.len() / 64;
        let key_stream = block(key, nonce, counter + i as u32);
        let input_block: &[u8] = &encrypted[i * 64..encrypted.len()];
        let encrypted_block = xor_serialized(&serialize_state(key_stream), &input_block);
        plaintext.extend_from_slice(&encrypted_block[0..encrypted.len() % 64]);
    }
    plaintext
}

pub fn block(key: [u32; 8], nonce: [u32; 3], counter: u32) -> [u32; 16] {
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
    }

    // Return the matrix summation of working state and initial state
    add_states(init_state, working_state)
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

fn xor_serialized(state1: &[u8], state2: &[u8]) -> Vec<u8> {
    // Calculate XOR between two states into a third
    let mut xor_state: Vec<u8> = Vec::new();
    for i in 0..std::cmp::min(state1.len(), state2.len()) {
        // TODO: Possibly find a way of
        // incorporaing bits that doesn't match
        // up in the final output, instead of
        // simply discarding them
        xor_state.push(state1[i] ^ state2[i]);
    }
    xor_state
}

fn serialize_state(state: [u32; 16]) -> [u8; 64] {
    // Serialize into bytes instead of u32
    let mut serialized: [u8; 64] = [0; 64];
    for i in 0..16 {
        serialized[4 * i..][..4].copy_from_slice(&state[i].to_le_bytes());
    }
    serialized
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
        let state = init_state(
            [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            [0x09000000, 0x4a000000, 0x00000000],
            1,
        );
        assert_eq!(
            state,
            [
                0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908,
                0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000,
                0x4a000000, 0x00000000
            ]
        );
    }

    #[test]
    fn add_states_1() {
        let state1 = [
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908,
            0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000,
            0x4a000000, 0x00000000,
        ];
        let state2 = [
            0x837778abu32,
            0xe238d763u32,
            0xa67ae21e,
            0x5950bb2f,
            0xc4f2d0c7,
            0xfc62bb2f,
            0x8fa018fc,
            0x3f5ec7b7,
            0x335271c2,
            0xf29489f3,
            0xeabda8fc,
            0x82e46ebd,
            0xd19c12b4,
            0xb04e16de,
            0x9e83d0cb,
            0x4e3c50a2,
        ];
        assert_eq!(
            add_states(state1, state2),
            [
                0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
                0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
                0xe883d0cb, 0x4e3c50a2
            ]
        );
    }

    #[test]
    fn xor_serialized_1() {
        let serialized_keystream = [
            0x22, 0x4f, 0x51, 0xf3, 0x40, 0x1b, 0xd9, 0xe1, 0x2f, 0xde, 0x27, 0x6f, 0xb8, 0x63,
            0x1d, 0xed, 0x8c, 0x13, 0x1f, 0x82, 0x3d, 0x2c, 0x06, 0xe2, 0x7e, 0x4f, 0xca, 0xec,
            0x9e, 0xf3, 0xcf, 0x78, 0x8a, 0x3b, 0x0a, 0xa3, 0x72, 0x60, 0x0a, 0x92, 0xb5, 0x79,
            0x74, 0xcd, 0xed, 0x2b, 0x93, 0x34, 0x79, 0x4c, 0xba, 0x40, 0xc6, 0x3e, 0x34, 0xcd,
            0xea, 0x21, 0x2c, 0x4c, 0xf0, 0x7d, 0x41, 0xb7, 0x69, 0xa6, 0x74, 0x9f, 0x3f, 0x63,
            0x0f, 0x41, 0x22, 0xca, 0xfe, 0x28, 0xec, 0x4d, 0xc4, 0x7e, 0x26, 0xd4, 0x34, 0x6d,
            0x70, 0xb9, 0x8c, 0x73, 0xf3, 0xe9, 0xc5, 0x3a, 0xc4, 0x0c, 0x59, 0x45, 0x39, 0x8b,
            0x6e, 0xda, 0x1a, 0x83, 0x2c, 0x89, 0xc1, 0x67, 0xea, 0xcd, 0x90, 0x1d, 0x7e, 0x2b,
            0xf3, 0x63,
        ];
        let serialized_plaintext = [
            0x4c, 0x61, 0x64, 0x69, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x47, 0x65, 0x6e,
            0x74, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
            0x63, 0x6c, 0x61, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x27, 0x39, 0x39, 0x3a, 0x20,
            0x49, 0x66, 0x20, 0x49, 0x20, 0x63, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x66,
            0x65, 0x72, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e,
            0x65, 0x20, 0x74, 0x69, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
            0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x2c, 0x20, 0x73, 0x75, 0x6e, 0x73, 0x63, 0x72,
            0x65, 0x65, 0x6e, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69,
            0x74, 0x2e,
        ];
        assert_eq!(
            xor_serialized(&serialized_keystream, &serialized_plaintext),
            &[
                0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d,
                0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc,
                0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59,
                0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
                0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d,
                0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e, 0x52, 0xbc, 0x51, 0x4d,
                0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9,
                0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
                0x87, 0x4d
            ]
        )
    }

    // #[test]
    // fn block_1() {
    //     let out = block([0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c], [0x09000000, 0x4a000000, 0x00000000], 1);
    //     assert_eq!(
    //         out,
    //         [0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20, 0x71, 0xc4, 0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a, 0xc3, 0xd4, 0x6c, 0x4e, 0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09, 0x14, 0xc2, 0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2, 0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9, 0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e]

    //     )
    // }

    #[test]
    fn block_and_serialize_1() {
        let out = serialize_state(block(
            [
                0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
                0x1f1e1d1c,
            ],
            [0x09000000, 0x4a000000, 0x00000000],
            1,
        ));
        assert_eq!(
            out,
            [
                0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20,
                0x71, 0xc4, 0xc7, 0xd1, 0xf4, 0xc7, 0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a,
                0xc3, 0xd4, 0x6c, 0x4e, 0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09, 0x14, 0xc2,
                0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2, 0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9,
                0xcb, 0xd0, 0x83, 0xe8, 0xa2, 0x50, 0x3c, 0x4e
            ]
        )
    }

    #[test]
    fn encrypt_1() {
        let plaintext = &[
            0x4cu8, 0x61u8, 0x64u8, 0x69u8, 0x65u8, 0x73u8, 0x20u8, 0x61u8, 0x6eu8, 0x64u8, 0x20u8,
            0x47u8, 0x65u8, 0x6eu8, 0x74u8, 0x6cu8, 0x65u8, 0x6du8, 0x65u8, 0x6eu8, 0x20u8, 0x6fu8,
            0x66u8, 0x20u8, 0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x63u8, 0x6cu8, 0x61u8, 0x73u8, 0x73u8,
            0x20u8, 0x6fu8, 0x66u8, 0x20u8, 0x27u8, 0x39u8, 0x39u8, 0x3au8, 0x20u8, 0x49u8, 0x66u8,
            0x20u8, 0x49u8, 0x20u8, 0x63u8, 0x6fu8, 0x75u8, 0x6cu8, 0x64u8, 0x20u8, 0x6fu8, 0x66u8,
            0x66u8, 0x65u8, 0x72u8, 0x20u8, 0x79u8, 0x6fu8, 0x75u8, 0x20u8, 0x6fu8, 0x6eu8, 0x6cu8,
            0x79u8, 0x20u8, 0x6fu8, 0x6eu8, 0x65u8, 0x20u8, 0x74u8, 0x69u8, 0x70u8, 0x20u8, 0x66u8,
            0x6fu8, 0x72u8, 0x20u8, 0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x66u8, 0x75u8, 0x74u8, 0x75u8,
            0x72u8, 0x65u8, 0x2cu8, 0x20u8, 0x73u8, 0x75u8, 0x6eu8, 0x73u8, 0x63u8, 0x72u8, 0x65u8,
            0x65u8, 0x6eu8, 0x20u8, 0x77u8, 0x6fu8, 0x75u8, 0x6cu8, 0x64u8, 0x20u8, 0x62u8, 0x65u8,
            0x20u8, 0x69u8, 0x74u8, 0x2eu8,
        ];
        let key = [
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
            0x1f1e1d1c,
        ];
        let nonce = [0x00000000, 0x4a000000, 0x00000000];
        let counter = 1;
        let encrypted_data = encrypt(plaintext, key, nonce, counter);
        assert_eq!(
            encrypted_data,
            [
                0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d,
                0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc,
                0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59,
                0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
                0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d,
                0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e, 0x52, 0xbc, 0x51, 0x4d,
                0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9,
                0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
                0x87, 0x4d
            ]
        )
    }

    #[test]
    fn decrypt_1() {
        let encrypted = &[
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d,
            0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc,
            0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59,
            0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
            0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d,
            0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e, 0x52, 0xbc, 0x51, 0x4d,
            0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9,
            0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
            0x87, 0x4d,
        ];
        let key = [
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918,
            0x1f1e1d1c,
        ];
        let nonce = [0x00000000, 0x4a000000, 0x00000000];
        let counter = 1;
        assert_eq!(
            decrypt(encrypted, key, nonce, counter),
            [
                0x4cu8, 0x61u8, 0x64u8, 0x69u8, 0x65u8, 0x73u8, 0x20u8, 0x61u8, 0x6eu8, 0x64u8,
                0x20u8, 0x47u8, 0x65u8, 0x6eu8, 0x74u8, 0x6cu8, 0x65u8, 0x6du8, 0x65u8, 0x6eu8,
                0x20u8, 0x6fu8, 0x66u8, 0x20u8, 0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x63u8, 0x6cu8,
                0x61u8, 0x73u8, 0x73u8, 0x20u8, 0x6fu8, 0x66u8, 0x20u8, 0x27u8, 0x39u8, 0x39u8,
                0x3au8, 0x20u8, 0x49u8, 0x66u8, 0x20u8, 0x49u8, 0x20u8, 0x63u8, 0x6fu8, 0x75u8,
                0x6cu8, 0x64u8, 0x20u8, 0x6fu8, 0x66u8, 0x66u8, 0x65u8, 0x72u8, 0x20u8, 0x79u8,
                0x6fu8, 0x75u8, 0x20u8, 0x6fu8, 0x6eu8, 0x6cu8, 0x79u8, 0x20u8, 0x6fu8, 0x6eu8,
                0x65u8, 0x20u8, 0x74u8, 0x69u8, 0x70u8, 0x20u8, 0x66u8, 0x6fu8, 0x72u8, 0x20u8,
                0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x66u8, 0x75u8, 0x74u8, 0x75u8, 0x72u8, 0x65u8,
                0x2cu8, 0x20u8, 0x73u8, 0x75u8, 0x6eu8, 0x73u8, 0x63u8, 0x72u8, 0x65u8, 0x65u8,
                0x6eu8, 0x20u8, 0x77u8, 0x6fu8, 0x75u8, 0x6cu8, 0x64u8, 0x20u8, 0x62u8, 0x65u8,
                0x20u8, 0x69u8, 0x74u8, 0x2eu8,
            ]
        );
    }

    #[test]
    fn testing_input_bytestream_1() {
        assert_eq!(
            b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.",
            &[0x4cu8, 0x61u8, 0x64u8, 0x69u8, 0x65u8, 0x73u8, 0x20u8, 0x61u8, 0x6eu8, 0x64u8, 0x20u8, 0x47u8, 0x65u8, 0x6eu8, 0x74u8, 0x6cu8, 0x65u8, 0x6du8, 0x65u8, 0x6eu8, 0x20u8, 0x6fu8, 0x66u8, 0x20u8, 0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x63u8, 0x6cu8, 0x61u8, 0x73u8, 0x73u8, 0x20u8, 0x6fu8, 0x66u8, 0x20u8, 0x27u8, 0x39u8, 0x39u8, 0x3au8, 0x20u8, 0x49u8, 0x66u8, 0x20u8, 0x49u8, 0x20u8, 0x63u8, 0x6fu8, 0x75u8, 0x6cu8, 0x64u8, 0x20u8, 0x6fu8, 0x66u8, 0x66u8, 0x65u8, 0x72u8, 0x20u8, 0x79u8, 0x6fu8, 0x75u8, 0x20u8, 0x6fu8, 0x6eu8, 0x6cu8, 0x79u8, 0x20u8, 0x6fu8, 0x6eu8, 0x65u8, 0x20u8, 0x74u8, 0x69u8, 0x70u8, 0x20u8, 0x66u8, 0x6fu8, 0x72u8, 0x20u8, 0x74u8, 0x68u8, 0x65u8, 0x20u8, 0x66u8, 0x75u8, 0x74u8, 0x75u8, 0x72u8, 0x65u8, 0x2cu8, 0x20u8, 0x73u8, 0x75u8, 0x6eu8, 0x73u8, 0x63u8, 0x72u8, 0x65u8, 0x65u8, 0x6eu8, 0x20u8, 0x77u8, 0x6fu8, 0x75u8, 0x6cu8, 0x64u8, 0x20u8, 0x62u8, 0x65u8, 0x20u8, 0x69u8, 0x74u8, 0x2eu8]
            );
    }
}
