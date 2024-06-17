use std::fs::File;
use std::io::Write;
use std::process::Command;

pub async fn run(
    jar_bytes: Vec<u8>,
    file_name: &str,
    jar_command: &str,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut file = File::create(file_name)?;
    file.write_all(&jar_bytes)?;

    let mut command = build_command(jar_command).await;
    command.spawn()?;
    command.wait()?;

    Ok(command)
}

async fn build_command(
    command: &str,
) -> Command {
    let command_args = command.split_whitespace().collect::<Vec<_>>();
    command_args.iter().fold(
        Command::new(command_args[0]),
        |mut command, arg| {
            command.arg(arg);
            command
        },
    )
}