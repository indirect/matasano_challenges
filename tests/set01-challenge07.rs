extern crate openssl;
extern crate serialize;

use serialize::base64::FromBase64;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

#[test]
fn set01_challenge07() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge07.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let base64 = reader.read_to_end().unwrap();
    let data = base64.from_base64().unwrap();

    let key = "YELLOW SUBMARINE".as_bytes();
    let iv = Vec::new();
    let result = openssl::crypto::symm::decrypt(
        openssl::crypto::symm::Type::AES_128_ECB,
        key.as_slice(),
        iv,
        data.as_slice()
    );
    let plaintext = String::from_utf8_lossy(result.as_slice());

    assert!(plaintext.contains("I'm back and I'm ringin' the bell"));
}
