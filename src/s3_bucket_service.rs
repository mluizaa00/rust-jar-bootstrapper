use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3 as s3;
use aws_sdk_s3::Client;
use crate::boot_instruction::{BootInstruction, S3Instructions};
use crate::file_service;

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_client: Client,
    pub instructions: S3Instructions,
}

pub async fn initialize_client() -> AppState {
    let boot_instruction = file_service::load_json_from_file::<BootInstruction>("config/bootable.json").await
        .expect("Unable to load credentials json.");

    let region_provider = RegionProviderChain::default_provider()
        .or_else("us-east-2");
    let config = aws_config::from_env()
        .region(region_provider)
        .load().await;

    let client = Client::new(&config);
    println!("Initialized AWS client.");

    AppState { aws_client: client, instructions: boot_instruction.s3.instructions }
}

pub async fn download_file_from_s3(
    client: &Client,
    bucket_name: &String,
    object_key: &String
) -> Result<Vec<u8>, s3::Error> {
    let response = client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await
        .expect("Unable to get object on S3.");

    let bytes = response.body.collect().await
        .expect("Unable to collect response body from S3.")
        .into_bytes();

    println!("Successfully downloaded file from s3 at bucket={}, key={}", bucket_name, object_key);
    Ok(bytes.to_vec())
}