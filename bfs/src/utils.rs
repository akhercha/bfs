use std::fmt::Write;

use bigdecimal::ToPrimitive;

pub const EMOJI_RANGE_START: u128 = 0x1F600;
pub const EMOJI_RANGE_END: u128 = 0x1F64F;
pub const EMOJI_RANGE: u128 = EMOJI_RANGE_END - EMOJI_RANGE_START + 1;

fn hash_to_emoji(hash: &String) -> String {
    let hash_value = u128::from_str_radix(&hash[2..32], 16).unwrap();
    let emoji_codepoint = EMOJI_RANGE_START + (hash_value % EMOJI_RANGE);
    char::from_u32(emoji_codepoint.to_u32().unwrap())
        .unwrap()
        .to_string()
}

pub fn format_hash(hash: &String) -> String {
    hash_to_emoji(&hash)
        + " 0x"
        + &hash[..8].to_string()
        + "..."
        + &hash[hash.len() - 4..hash.len()]
}

pub fn bytes_to_hash(bytes: &[u8]) -> String {
    let mut hash = String::new();
    for byte in bytes {
        write!(&mut hash, "{:02x}", byte).expect("Unable to write");
    }
    format!("{}", format_hash(&hash))
}
