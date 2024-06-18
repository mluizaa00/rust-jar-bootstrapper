use std::process::Command;

pub async fn run(
    jar_command: &String,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut command = build_command(jar_command).await;
    command.spawn()?;

    println!("Successfully ran command={}", jar_command);
    Ok(command)
}

async fn build_command(
    command: &String,
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