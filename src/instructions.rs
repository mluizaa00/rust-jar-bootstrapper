use serde_json::from_str;
use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Instructions {
    pub instructions: Instruction,
}

#[derive(Debug, Deserialize)]
struct Instruction {
    pub boot_command: str,
    pub bucket_name: str,
    pub object_key: str,
}

pub async fn load_instructions(
    file_path: &str
) -> Result<Instructions, Box<dyn std::error::Error>> {
    let json_data = read_to_string(file_path)?;
    let instructions: Instructions = from_str(&json_data)?;

    Ok(instructions)
}