pub struct Cryptstream {
    path: String, // Path to file
    data: Vec<u8>, // Data buffer
    open: bool, // File open or closed
}

impl Cryptstream {
    pub fn open_cryptstream(path: String, passphrase: String) -> Result<Cryptstream, ()> {
        
        todo!("Opening cryptstream not implemented yet");
    }
    pub fn close_cryptstream(&mut self) {

        todo!("Closing cryptstream not implemented yet");
    }
    pub fn write(&mut self, data: String) {

        todo!("Writing to cryptstream not implemented yet");
    }
    pub fn flush(&mut self) -> Result<(),()> {

        todo!("Flushing cryptstream not implemented yet");
    }
}

