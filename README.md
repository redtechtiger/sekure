# Sekure
This is a high performant file encryption library for Rust, safe from side channel attacks through the use of the newer ChaCha20 encryption algorithm.
> ⚠️ This project is under development and several features have not yet been implemented! Certain functions are NOT constant time and performance optimization is still underway. This is NOT ready for use in a production or otherwise sensitive environment. Check the roadmap!

## Usage
Sekure is available as a crate from Cargo. Example usage:
```
let mut cryptstream = Cryptstream::open_cryptstream("my_secret.skr","PA$$WORD").unwrap();
cryptstream.write("Hello, world!");
cryptstream.close_cryptstream();
```

## Contributing
This code is licensed under the MIT license, and as always users are more than welcome to submit bug reports or pull requests.

## References
This library was designed according to the specifications described in [RFC 7539](https://datatracker.ietf.org/doc/html/rfc7539) as well as [RFC 8439](https://datatracker.ietf.org/doc/html/rfc8439).
