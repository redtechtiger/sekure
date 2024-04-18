pub struct Cryptstream {
    path: String, // Path to file
    data: Vec<u8>, // Data buffer
    open: bool, // File open or closed
}

impl Cryptstream {
    pub fn open_cryptstream(path: String, passphrase: String) -> Result<Cryptstream, ()> {
        
        todo!(); // Decrypt and read data
    }
    pub fn close_cryptstream(&mut self) {

        todo!(); // Close and secure memory
    }
    pub fn write(&mut self, data: String) {

        todo!(); // Write data to memory
    }
    pub fn flush(&mut self) -> Result<(),()> {

        todo!(); // Write encrypted data to disk
    }
}

