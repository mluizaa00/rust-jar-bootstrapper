use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BootInstruction {
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