pub use crypt_utils::{wrapped_shift_letters, KeyGenerator};

pub fn encrypt_str(to_encrypt: &str, mut keygen: KeyGenerator) -> String {
    let mut source_chars = to_encrypt.chars();
    let mut result_chars: Vec<char> = Vec::new();
    for _ in 0..source_chars.as_str().len() {
        let to_push = wrapped_shift_letters(source_chars.next().unwrap(), keygen.next().unwrap());
        result_chars.push(to_push);
    }
    result_chars.into_iter().collect()
}
