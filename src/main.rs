extern crate base64;
extern crate hex;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

fn main() {
    let key = "this_is_key".as_bytes();
    let mut hasher = Sha1::new();
    hasher.input(key);
    let sha1_string = hasher.result_str();
    // sha1_string: 558c6e2f93212d10f8b4ab1ac77031e2ba157471
    let bytes = hex::decode(sha1_string).unwrap();
    let sha1_base64 = base64::encode(bytes);
    println!("{:?}", sha1_base64);
}