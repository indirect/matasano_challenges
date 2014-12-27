extern crate decrypt;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

#[test]
fn set01_challenge04() {
    let path = Path::new(file!().to_string() + "/../../files/set01-challenge04.txt");
    let mut reader = BufferedReader::new(File::open(&path).unwrap());
    let answers = reader.lines().filter_map(|line| decrypt::xor_byte(line.unwrap().as_slice()) );
    let best = answers.max_by(|answer| answer.score ).unwrap();

    assert_eq!(best.text, "Now that the party is jumping\n");
    assert_eq!(best.key, "5");
}