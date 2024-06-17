mod aws_download;
mod run_jar;
mod instructions;

use aws_sdk_s3 as s3;
use aws_download::download_jar_from_s3;
use run_jar::run;
use instructions::load_instructions;

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let instructions = load_instructions("/config/instructions.json").await
        .expect("Unable to load instructions json.");

    let jar = download_jar_from_s3(
        Box::new(instructions.instructions.bucket_name),
        Box::new(instructions.instructions.object_key)
    ).await
        .expect("Unable to download jar from S3.");

    run(jar, "file_name", "command").await
        .expect("Unable to run jar file.");

    Ok(())
}