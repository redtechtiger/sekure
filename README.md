# Sekure
This is a high performant file encryption library for Rust, safe from side channel attacks through the use of the newer ChaCha20 encryption algorithm.

## Usage
This code is available as a crate on Cargo. Example usage:
```
let mut cryptstream = Cryptstream::open_cryptstream("my_secret.skr","PA$$WORD").unwrap();
cryptstream.write("Hello, world!");
// cryptstream.flush(); // Optional, as it gets flushed automatically when closing
cryptstream.close_cryptstream();
```

## Licensing
This code is licensed under the MIT license, and as always users are more than welcome to submit bug reports or pull requests.

