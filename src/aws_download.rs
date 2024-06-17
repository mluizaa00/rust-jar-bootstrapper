use aws_sdk_s3 as s3;
use aws_sdk_s3::config::Region;

pub async fn download_jar_from_s3(
    bucket_name: Box<str>,
    object_key: Box<str>
) -> Result<Vec<u8>, s3::Error> {
    let config = aws_config::from_env()
        .region(Region::default());
    let client = s3::Client::new(&config);

    let get_object_request = GetObjectRequest::builder()
        .bucket(bucket_name)
        .key(object_key)
        .build();

    let response = client.get_object(get_object_request).await?;

    let mut downloaded_data = vec![];
    let mut stream = response.body.unwrap();
    while let Some(Ok(chunk)) = stream.next().await {
        downloaded_data.extend_from_slice(&chunk);
    }

    Ok(downloaded_data)
}