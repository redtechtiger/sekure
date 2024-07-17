# Sekure
This is an intuitive & performant file encryption library for Rust, providing enhanced protection against side-channel attacks. It includes a simple yet powerful API to effortlessly write highly protected files secured by a user password. Under the hood, Sekure is powered by ChaCha20, Poly1305, and PBKDF2. These algorithms have been specifically picked out due to their low performance overhead combined with strong security.
> ⚠️ This project is under development and several features have not yet been implemented! Side channel resistance is not guaranteed and performance optimizations are still underway. This is NOT ready for use in a production or otherwise sensitive environment. Check the roadmap!

## Usage
Sekure will be available as a crate from Cargo once basic functionality is complete. Example usage:
```
// Define our password and file location
let filename = "my_file.txt"
let password = "foobar2024";

// Open the file
let mut my_encrypted_file = CryptStream::open(filename, password).unwrap();

// Write encrypted data to it
my_encrypted_file.write("Hello, world!");

// Save file and safely destroy internal library data.
my_encrypted_file.close();
```

## Contributing
This code is licensed under the MIT license, and as always users are more than welcome to submit bug reports or pull requests.

## References
This library was designed according to the specifications described in [RFC 7539](https://datatracker.ietf.org/doc/html/rfc7539), [RFC 8439](https://datatracker.ietf.org/doc/html/rfc8439), and [NIST Special Publication 800-132](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-132.pdf).
