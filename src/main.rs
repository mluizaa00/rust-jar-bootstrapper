mod s3_bucket_service;
mod run_jar;
mod instructions;

use aws_sdk_s3 as s3;
use s3_bucket_service::download_jar_from_s3;
use run_jar::run;
use instructions::load_instructions;
use crate::s3_bucket_service::initialize_client;

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let instructions = load_instructions("/config/instructions.json").await
        .expect("Unable to load instructions json.");

    let state = initialize_client(Box::new(instructions.s3_configuration))
        .await.expect("Unable to initialize AWS client.");

    let jar = download_jar_from_s3(
        state.aws_client,
        Box::new(instructions.s3_configuration.bucket),
        Box::new(instructions.s3_configuration.object_key)
    ).await
        .expect("Unable to download jar from S3.");

    run(jar, "file_name", "command").await
        .expect("Unable to run jar file.");

    Ok(())
}