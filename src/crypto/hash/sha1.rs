use hex_literal::hex;
use sha1::{Digest, Sha1};
use std::hash::Hash;

#[test]
fn sha1() {
    let mut hasher = Sha1::new();
    hasher.update(b"hello world");
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));

    let hex_string = hex::encode(result);
    let decoded_vec = hex::decode("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed").unwrap_or_default();
    assert_eq!(result[..], decoded_vec);

    println!("{}", hex_string);
    println!("{:?}", result);
}

#[test]
fn sha1_file() {
    let text = std::fs::read_to_string("data/index.m3u8").unwrap();
    println!("{}", text);

    let bytes = std::fs::read("data/index.m3u8").unwrap();
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let hex_str = hex::encode_upper(result);
    assert_eq!(hex_str, "105230C30784ACA38D33AF6E077D5982F04F936C");
    println!("sha1: {}", hex_str);
    println!("file:\n{}", text);
}
