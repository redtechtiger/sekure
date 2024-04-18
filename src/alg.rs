
pub struct Cryptstream {
    path: String,
}

impl Cryptstream {
    pub fn open_cryptstream(path: String, passphrase: String) -> Result<Cryptstream, ()> {
        
        todo!(); // Decrypt and read data
    }
    pub fn close_cryptstream(&mut self) {

        todo!(); // Close and secure memory
    }
    pub fn write(data: String) {

        todo!(); // Write data to memory
    }
    pub fn flush() -> Result<(),()> {

        todo!(); // Write encrypted data to disk
    }
}

