use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::error::Error;

pub fn serialize_to_file<T: Serialize, P: AsRef<Path>>(data: T, file_path: P) -> Result<(), Error> {
    let json = serde_json::to_string(&data)?;

    let mut file = File::create(file_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn deserialize_from_file<T: DeserializeOwned, P: AsRef<Path>>(file_path: P) -> Result<T, Error> {
    let mut file = File::open(file_path)?;

    let mut json = String::new();
    file.read_to_string(&mut json)?;

    let data = serde_json::from_str::<T>(json.as_str())?;

    Ok(data)
}