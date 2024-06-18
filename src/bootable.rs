use serde_json::from_str;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Bootable {
    pub s3: S3Configuration,
}

#[derive(Debug, Deserialize)]
pub struct S3Configuration {
    pub instructions: S3Instructions,
}

#[derive(Debug, Deserialize, Clone)]
pub struct S3Instructions {
    pub bucket: String,
    pub key: String,
}

pub async fn load_credentials(
    file_path: &str
) -> Result<Bootable, Box<dyn std::error::Error>> {
    let json_data = read_to_string(file_path)?;
    let instructions = from_str(&json_data)?;

    Ok(instructions)
}