use std::ops::Add;

#[derive(Debug)]
pub struct BigU288([u8; 36]); // 288 bit unsigned integer (8x36)

impl Add for BigU288 {
    type Output = BigU288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = BigU288::new();
        for (index, byte) in self.0.iter().enumerate() {
            // LSB first
            let original_result_byte = output.0[index];
            output.0[index] += byte.wrapping_add(other.0[index]);
            let carry = (*byte as u16 + other.0[index] as u16 + original_result_byte as u16)
                .checked_sub(output.0[index] as u16)
                .unwrap_or(0);
            dbg!(carry, carry / 256);
            output.0[std::cmp::min(index + 1, output.0.len() - 1)] = (carry / 256) as u8;
        }
        output
    }
}

impl PartialEq for BigU288 {
    fn eq(&self, other: &BigU288) -> bool {
        self.0 == other.0
    }
}
impl Eq for BigU288 {}

impl BigU288 {
    pub fn add_msb(&mut self) {
        // TODO: Important! Attempt to solve this in constant time
        let mut i: bool = false; // Flag to see if we've hit the msb yet
        for (index, byte) in self.0.iter().rev().enumerate() { // Enumerate backwards (msb first)
             // let bit_1 = byte & 0b1000_0000; // Shift
             // let bit_1 = byte & 0b0000_0001; // Shift
        }
        todo!("msb isn't implemented yet");
    }
    pub fn from_slice(bytes: &[u8]) -> BigU288 {
        let mut big_u288 = BigU288::new();
        big_u288.0 = pad_array(bytes.to_vec()).as_slice().try_into().unwrap();
        big_u288
    }
    pub fn from_hex(input: &str) -> BigU288 {
        let mut big_u288 = BigU288::new();
        // Iterate over the string backwards (we want little endian)
        let input_padded_le = pad_array(input.bytes().rev().collect());
        for (index, char) in input_padded_le.iter().enumerate() {
            let hex_digit = u8::from_str_radix(
                &String::from_utf8(vec![*char]).unwrap_or("0".to_string()),
                16,
            )
            .unwrap_or(0);
            big_u288.0[index / 2] += hex_digit << 4 * (index % 2);
        }
        big_u288
    }
    pub fn get_bytes(&self) -> [u8; 36] {
        self.0
    }
    pub fn new() -> BigU288 {
        BigU288([0; 36])
    }
}

fn pad_array(input: Vec<u8>) -> Vec<u8> {
    let mut padded = [0u8; 72]; // TODO: Make this configurable
    padded[..input.len()].copy_from_slice(&input);
    padded.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_msb_1() {
    // }

    // #[test]
    // fn from_hex_1() {
    //     let big_u288 = BigU288::from_hex("3fffffffffffffffffffffffffffffffb");
    //     assert_eq!(
    //         BigU288::from_hex()
    //     )
    // }

    #[test]
    fn from_hex_2() {
        assert_eq!(
            BigU288::from_hex("f").get_bytes(),
            [
                15u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn from_slice_1() {
        assert_eq!(
            BigU288::from_slice(&[1,1]),
            BigU288::from_hex("F")
        );
    }

    #[test]
    fn from_hex_3() {
        assert_eq!(
            BigU288::from_hex("ff").get_bytes(),
            [
                255u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn from_hex_4() {
        assert_eq!(
            BigU288::from_hex("1fff").get_bytes(),
            [
                255u8, 31u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    // #[test]
    // fn add_msb_1() {
    //     let mut big_u288 = BigU288::from_hex("9");
    //     big_u288.add_msb();
    // }

    #[test]
    fn add_1() {
        assert_eq!(
            BigU288::from_hex("1") + BigU288::from_hex("ff"),
            BigU288::from_hex("100")
        );
    }

    #[test]
    fn add_2() {
        assert_eq!(
            BigU288::from_hex("C1583054D5A6350B37E23A")
                + BigU288::from_hex("2A677ACE04C0037CA98B6BC"),
            BigU288::from_hex("367cfdd3521a66cd5d098f6")
        );
    }

    #[test]
    fn pad_array_1() {
        assert_eq!(
            pad_array(vec![255, 255]),
            vec![
                255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
