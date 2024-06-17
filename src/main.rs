mod s3_bucket_service;
mod module_service;
mod instructions;

use aws_sdk_s3 as s3;
use s3_bucket_service::download_jar_from_s3;
use module_service::run;
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
        instructions.s3_configuration.modules.bucket,
        instructions.s3_configuration.modules.key
    ).await
        .expect("Unable to download jar from S3.");

    let command = run(jar, "jar_module", &instructions.instructions.boot_command).await
        .expect("Unable to run jar file.");

    Ok(())
}