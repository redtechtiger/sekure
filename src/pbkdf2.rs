pub fn derive_cryptographic_key<const A: usize>(
    password: &str,
    salt: [u8; 128],
    iteration_count: usize,
) -> [u8; A] {
    todo!();
}
