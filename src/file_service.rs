use std::fs::read_to_string;
use serde::de::DeserializeOwned;
use serde_json::from_str;

pub async fn load_json_from_file<T>(
    file_path: &str
) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned {
    let json_data = read_to_string(file_path)?;
    Ok(from_str(&json_data)?)
}