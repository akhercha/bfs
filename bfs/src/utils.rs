use std::fmt::Write;

use bigdecimal::ToPrimitive;
use k256::ecdsa::VerifyingKey;

use crate::transaction::Transaction;

pub const EMOJI_RANGE_START: u128 = 0x1F600;
pub const EMOJI_RANGE_END: u128 = 0x1F64F;
pub const EMOJI_RANGE: u128 = EMOJI_RANGE_END - EMOJI_RANGE_START + 1;

fn hash_to_emoji(hash: &str) -> String {
    let hash_value = u128::from_str_radix(&hash[2..32], 16).unwrap();
    let emoji_codepoint = EMOJI_RANGE_START + (hash_value % EMOJI_RANGE);
    char::from_u32(emoji_codepoint.to_u32().unwrap())
        .unwrap()
        .to_string()
}

pub fn to_readable_hash(hash: &str) -> String {
    hash_to_emoji(hash) + " " + &hash[..8] + "..." + &hash[hash.len() - 4..hash.len()]
}

pub fn bytes_to_hash(bytes: &[u8]) -> String {
    let mut hash = String::new();
    for byte in bytes {
        write!(&mut hash, "{:02x}", byte).expect("Unable to write");
    }
    format!("0x{}", hash)
}

pub fn hash_to_bytes(hash: &str) -> Vec<u8> {
    let hash = if let Some(stripped_hash) = hash.strip_prefix("0x") {
        stripped_hash
    } else {
        hash
    };

    (0..hash.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash[i..i + 2], 16).expect("Invalid hex string"))
        .collect()
}

pub fn verifying_key_to_string(key: &VerifyingKey) -> String {
    bytes_to_hash(key.to_encoded_point(true).as_bytes())
}

pub fn get_rand_txs(n: usize) -> Vec<Transaction> {
    (0..n).map(|_| rand::random::<Transaction>()).collect()
}

pub fn check_prefix(string: &str, letter: char, n: usize) -> bool {
    if string.len() < n {
        return false;
    }
    string.chars().take(n).all(|c| c == letter)
}
