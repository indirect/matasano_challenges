#![allow(unstable)]
pub mod ecb;
pub mod cbc;
pub mod pkcs7;

pub fn repeated_block_count(bytes: &[u8], block_size: usize) -> usize {
    let block_count = bytes.len() / block_size as usize;

    let mut repeated_blocks: usize = 0;
    for block_idx in range(0, block_count) {
        let block = bytes.slice(block_idx * block_size, (block_idx + 1) * block_size);
        for other_idx in range(block_idx + 1, block_count) {
            if block == bytes.slice(other_idx*block_size, (other_idx+1)*block_size) {
                repeated_blocks += 1;
            }
        }
    }

    repeated_blocks
}

#[test]
fn no_repeated_blocks() {
    let bytes = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(0, repeated_block_count(bytes.as_slice(), 3));
}

#[test]
fn one_repeated_block() {
    let bytes = vec![1, 1, 2, 3, 4, 5, 1, 1, 6, 7];
    assert_eq!(1, repeated_block_count(bytes.as_slice(), 2));
}

#[test]
fn three_repeated_blocks() {
    let bytes = vec![1, 1, 2, 3, 1, 1, 4, 5, 1, 1, 6, 7];
    assert_eq!(3, repeated_block_count(bytes.as_slice(), 2));
}


#[derive(Copy, PartialEq, Eq)]
pub enum Cipher {
    ECB,
    CBC
}

impl std::fmt::Show for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            &Cipher::ECB => write!(f, "ECB"),
            &Cipher::CBC => write!(f, "CBC")
        }
    }
}

pub fn detect_cipher(data: &[u8]) -> Cipher {
    match ecb::detect(data) {
        true => Cipher::ECB,
        false => Cipher::CBC
    }
}

#[test]
fn test_detect_cipher_with_ecb() {
    let mut plain: Vec<u8> = Vec::with_capacity(256);
    for b in range(0, 240) { plain.push(b % 16); }
    let cipher = ecb::encrypt(&b"YELLOW SUBMARINE"[], &plain[]);
    match detect_cipher(&cipher[]) {
        Cipher::ECB => (),
        Cipher::CBC => panic!("Oh no! detect_cipher is wrong")
    };
}

#[test]
fn test_detect_cipher_with_cbc() {
    let mut plain: Vec<u8> = Vec::with_capacity(256);
    for b in range(0, 240) { plain.push(b % 16); }
    let cipher = cbc::encrypt_zero_iv(&b"YELLOW SUBMARINE"[], &plain[]);
    match detect_cipher(&cipher[]) {
        Cipher::ECB => panic!("Oh no! detect_cipher is wrong"),
        Cipher::CBC => ()
    };
}


fn random_bytes(count: usize) -> Vec<u8> {
    use std::rand::{thread_rng, Rng};
    thread_rng().gen_iter::<u8>().take(count).collect()
}

fn random_padding() -> Vec<u8> {
    use std::rand::{thread_rng, Rng};
    random_bytes(thread_rng().gen_range::<usize>(5, 10))
}

pub fn random_key() -> Vec<u8> {
    random_bytes(16)
}

pub fn oracle(key: &[u8], input: &[u8]) -> Vec<u8> {
    let mut plain: Vec<u8> = Vec::new();

    plain.push_all(&random_padding()[]);
    plain.push_all(input);
    plain.push_all(&random_padding()[]);
    plain = pkcs7::pad(&plain[], 16);

    if std::rand::random() {
        ecb::encrypt(key, &plain[])
    } else {
        cbc::encrypt(key, &random_key()[], &plain[])
    }
}

#[test]
fn test_oracle() {
    let output = oracle(&random_key()[], &b"YELLOW SUBMARINE"[]);
    // no idea how to test anything other than length
    assert!(output.len() >= 16);
    assert!(output.len() <= 32);
}

pub fn random_oracle(input: &[u8]) -> Vec<u8> {
    oracle(&random_key()[], &input[])
}
