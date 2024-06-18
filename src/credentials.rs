use serde_json::from_str;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub s3: S3ConfigurationCredentials,
}

#[derive(Debug, Deserialize)]
pub struct S3ConfigurationCredentials {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: String,
    pub provider: String,
}

pub async fn load_credentials(
    file_path: &str
) -> Result<Credentials, Box<dyn std::error::Error>> {
    let json_data = read_to_string(file_path)?;
    let instructions = from_str(&json_data)?;

    Ok(instructions)
}