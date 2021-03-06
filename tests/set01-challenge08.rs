extern crate "rustc-serialize" as serialize;
extern crate crypto;

use serialize::hex::FromHex;
use std::io::BufferedReader;
use std::io::fs::File;
use std::path::Path;

#[test]
fn set01_challenge08() {
    let path = Path::new(file!().to_string() + "/../../data/set01-challenge08.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());

    let ecb_lines: Vec<usize> = reader.lines().enumerate().filter_map(|tuple| {
        let (index, hex_line) = tuple;
        let bytes = hex_line.unwrap().from_hex().unwrap();
        let is_ecb = crypto::ecb::detect(bytes.as_slice());

        if is_ecb {
            Some(index)
        } else {
            None
        }
    }).collect();

    // Line 132 has repeating 16 byte sections, and is therefore probably ECB
    assert_eq!(vec![132], ecb_lines);
}
