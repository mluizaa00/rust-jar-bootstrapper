use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Instructions {
    pub instructions: JarStartupInstruction,
}

#[derive(Debug, Deserialize)]
pub struct JarStartupInstruction {
    #[serde(rename(deserialize = "bootCommand"))]
    pub boot_command: String,
    #[serde(rename(deserialize = "fileName"))]
    pub file_name: String,
    pub bucket: String,
    pub key: String,
}