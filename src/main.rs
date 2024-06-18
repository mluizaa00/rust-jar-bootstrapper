mod s3_bucket_service;
mod command_service;
mod instructions;
mod bootable;
mod module_service;

use std::collections::HashMap;
use aws_sdk_s3 as s3;
use command_service::run;
use crate::module_service::download_and_create_json;
use crate::s3_bucket_service::initialize_client;

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let state = initialize_client().await;

    let instructions = download_and_create_json(&state).await;
    module_service::download_and_create_jar(state, &instructions).await;

    let command = run(&instructions.instructions.boot_command).await
        .expect("Unable to run jar file.");

    let mut commands = HashMap::new();
    commands.insert(&instructions.instructions.file_name, command);

    Ok(())
}