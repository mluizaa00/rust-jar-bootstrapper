use aws_sdk_s3 as s3;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{Credentials, Region};
use crate::instructions::S3Configuration;

#[derive(Clone, Debug)]
pub struct AppState {
    pub aws_client: Client,
}

pub async fn initialize_client(s3_configuration: Box<S3Configuration>) -> Result<AppState, s3::error> {
    let credentials = Credentials::from_keys(
        s3_configuration.credentials.access_key_id,
        s3_configuration.credentials.access_key_secret,
        None
    );

    let configuration = aws_config::from_env()
        .endpoint_url(s3_configuration.endpoint)
        .region(Region::new(s3_configuration.credentials.region))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&configuration);
    let state = AppState { aws_client: client };

    Ok(state)
}

pub async fn download_jar_from_s3(
    client: Client,
    bucket_name: String,
    object_key: String
) -> Result<Vec<u8>, s3::Error> {
    let response = client
        .put_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;

    let body = response.collect().await?.to_vec();

    let mut downloaded_data = vec![];
    let mut stream = body.unwrap();
    while let Some(Ok(chunk)) = stream.next().await {
        downloaded_data.extend_from_slice(&chunk);
    }

    Ok(downloaded_data)
}