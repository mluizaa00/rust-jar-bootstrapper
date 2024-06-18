use serde_json::from_str;
use std::str::from_utf8;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instructions {
    pub instructions: BootInstruction,
}

#[derive(Debug, Deserialize)]
pub struct BootInstruction {
    #[serde(rename(deserialize = "bootCommand"))]
    pub boot_command: String,
    #[serde(rename(deserialize = "fileName"))]
    pub file_name: String,
    pub bucket: String,
    pub key: String,
}

pub async fn load_instruction_from_bytes(
    bytes: &Vec<u8>
) -> Result<Instructions, Box<dyn std::error::Error>> {
    let json_stringify = from_utf8(&bytes)
        .expect("Unable to translate bytes into UTF-8 string.");

    let instructions = from_str(&json_stringify)?;
    Ok(instructions)
}