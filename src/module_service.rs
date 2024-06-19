use std::fs::File;
use std::io::Write;
use std::str::from_utf8;
use serde::de::DeserializeOwned;

use serde_json::from_str;

use crate::instructions::Instructions;
use crate::s3_bucket_service::{AppState, download_file_from_s3};

pub async fn download_and_create_jar(
    state: AppState,
    instructions: &Instructions
) {
    let jar_in_bytes = download_file_from_s3(
        &state.aws_client,
        &instructions.instructions.bucket,
        &instructions.instructions.key,
    ).await.expect("Unable to download instructions.json from S3.");

    create_jar_file(
        &instructions.instructions.file_name,
        &jar_in_bytes,
    ).await;
}
pub async fn create_jar_file(
    file_name: &String,
    jar_bytes: &Vec<u8>,
) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(&jar_bytes).unwrap();

    println!("Created file with name={}.", file_name);
}

pub async fn download_and_create_json<T>(state: &AppState) -> T where T: DeserializeOwned {
    let instructions_json_in_bytes = download_file_from_s3(
        &state.aws_client,
        &state.instructions.bucket,
        &state.instructions.key,
    ).await.expect("Unable to download instructions.json from S3.");

    load_json_from_bytes::<T>(&instructions_json_in_bytes).await
        .expect("Unable to translate instructions received from S3 to JSON.")
}

pub async fn load_json_from_bytes<T>(
    bytes: &[u8]
) -> Result<T, Box<dyn std::error::Error>> where T: DeserializeOwned {
    let json_string_value = from_utf8(bytes)?;
    let result = from_str(&json_string_value)?;
    Ok(result)
}