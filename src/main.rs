extern crate base64;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

fn main() {
    emit_as_base64();
}

/**
 * @returns emit_as_base64() -> dGhpc19pc19rZXk=
 */
fn emit_as_base64() {
    let key = "this_is_key".as_bytes();
    let mut hasher = Sha1::new();
    hasher.input(key);
    let sha1_string = hasher.result_str();
    // sha1_string: 558c6e2f93212d10f8b4ab1ac77031e2ba157471
    let chars = String::from(sha1_string).chars().collect::<Vec<_>>();
    let bytes = chars
        .chunks(2)
        .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap())
        .collect::<Vec<_>>();
    let sha1_base64 = base64::encode(bytes);
    println!("{:?}", sha1_base64);
}
