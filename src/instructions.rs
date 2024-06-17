use serde_json::from_str;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Instructions {
    pub instructions: Instruction,
    pub s3_configuration: S3Configuration,
}

#[derive(Debug, Deserialize)]
struct Instruction {
    pub boot_command: str,
}

#[derive(Debug, Deserialize)]
pub struct S3Configuration {
    pub credentials: S3ConfigurationCredentials,
    pub endpoint: str,
    pub bucket: str,
    pub object_key: str,
}

#[derive(Debug, Deserialize)]
pub struct S3ConfigurationCredentials {
    pub access_key_id: str,
    pub access_key_secret: str,
    pub region: str,
}

pub async fn load_instructions(
    file_path: &str
) -> Result<Instructions, Box<dyn std::error::Error>> {
    let json_data = read_to_string(file_path)?;
    let instructions: Instructions = from_str(&json_data)?;

    Ok(instructions)
}