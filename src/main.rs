mod caesar;
use caesar::encrypt_str;
use crypt_utils::KeyGenerator;
fn main() {
    let plaintext = "attack at dawn";
    let key = "queen";
    let keygen = KeyGenerator::new(String::from(key), true);
    let cyphertext: String = encrypt_str(plaintext, keygen);
    println!("{}", String::from(cyphertext));
}
