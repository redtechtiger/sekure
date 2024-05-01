use std::ops::Add;


#[derive(Debug)]
pub struct BigU288([u8; 36]); // 288 bit unsigned integer (8x36)


impl Add for BigU288 {
    type Output = BigU288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = BigU288::new();
        for byte in self.0 {
            
        };
        BigU288([0; 36])
    }
}

impl BigU288 {
    fn add_msb(&mut self) {
        todo!();
    }
    fn from_slice(bytes: &[u8]) -> BigU288 {
        todo!();
    }
    fn from_hex(input: &str) -> BigU288 {
        let mut big_u288 = BigU288::new();
        // Iterate over the string backwards (we want little endian)
        for (index, char) in input.chars().rev().enumerate() {
            big_u288.0[index] = u8::from_str_radix(&char.to_string(), 16).expect("invalid character found");
        };
        big_u288
    }
    fn new() -> BigU288 {
        BigU288([0; 36])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn add_msb_1() {
    // }
    
    #[test]
    fn from_hex_1() {
        let big_u288 = BigU288::from_hex("3fffffffffffffffffffffffffffffffb");
        dbg!(big_u288);
    }
}
