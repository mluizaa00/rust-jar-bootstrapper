mod s3_bucket_service;
mod command_service;
mod instructions;
mod boot_instruction;
mod module_service;
mod file_service;

use aws_sdk_s3 as s3;
use command_service::run;
use crate::instructions::Instructions;
use crate::module_service::download_and_create_json;
use crate::s3_bucket_service::initialize_client;

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let state = initialize_client().await;

    let instructions = download_and_create_json::<Instructions>(&state).await;
    module_service::download_and_create_jar(state, &instructions).await;

    let _command = run(&instructions.instructions.boot_command).await
        .expect("Unable to run jar file.");

    Ok(())
}