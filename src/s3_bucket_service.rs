use aws_sdk_s3 as s3;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};
use crate::credentials::S3ConfigurationCredentials;
use crate::instructions::S3Configuration;

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_client: Client,
}

pub async fn initialize_client(
    s3_configuration: &S3Configuration,
    s3_credentials: S3ConfigurationCredentials,
) -> AppState {
    let credentials = Credentials::new(
        s3_credentials.access_key_id,
        s3_credentials.access_key_secret,
        None,
        None,
        "rust-bootstrapper",
    );

    let configuration = aws_config::from_env()
        .endpoint_url(&s3_configuration.endpoint)
        .region(Region::new(s3_credentials.region))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&configuration);
    AppState { aws_client: client }
}

pub async fn download_file_from_s3(
    client: Client,
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
    Ok(bytes.to_vec())
}