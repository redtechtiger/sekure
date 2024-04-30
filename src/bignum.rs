use std::ops::Add;

pub struct BigU288([u8;36]); // 288 bit unsigned integer (8x36)

impl Add for BigU288 {
    type Output = BigU288;
    fn add(self, other: Self) -> Self::Output {
        let mut output = BigU288::new();
        for byte in self.0 {
            
        }
        BigU288([0;36])
    }
}

impl BigU288 {
    fn add_msg(&mut self) {
        todo!();
    }
    fn from_slice(byets: &[u8]) -> BigU288 {
        todo!();
    }
    fn new() -> BigU288 {
        BigU288([0;36])
    }
}

