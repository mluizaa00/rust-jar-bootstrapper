use serde_json::from_str;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instructions {
    pub boot: BootInstruction,
    pub s3: S3Configuration,
}

#[derive(Debug, Deserialize)]
pub struct BootInstruction {
    pub boot_command: String,
    pub file_name: String,
}

#[derive(Debug, Deserialize)]
pub struct S3Configuration {
    pub endpoint: String,
    pub modules: S3ModulesConfiguration,
}

#[derive(Debug, Deserialize)]
pub struct S3ModulesConfiguration {
    pub bucket: String,
    pub key: String,
}

pub async fn load_instructions(
    file_path: &str
) -> Result<Instructions, Box<dyn std::error::Error>> {
    let json_data = read_to_string(file_path)?;
    let instructions = from_str(&json_data)?;

    Ok(instructions)
}