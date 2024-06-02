use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::{
    fs::{File, OpenOptions},
    io::Read,
};

use crate::utils::to_readable_hash;

pub trait Hashable: Serialize {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    fn get_hash(&self) -> String {
        sha256::digest(self.to_bytes())
    }

    fn get_readable_hash(&self) -> String {
        to_readable_hash(&self.get_hash())
    }

    fn from_json_file<T>(file_path: &str) -> std::io::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut file = File::open(file_path)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;
        let new: T = serde_json::from_str(&json_data)?;
        Ok(new)
    }

    fn to_json_file(&self, new_file: &str) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(new_file)
            .unwrap();
        let json_str = self.to_json();
        let json_value: Value = from_str(&json_str).unwrap();
        serde_json::to_writer_pretty(file, &json_value).unwrap();
    }
}
