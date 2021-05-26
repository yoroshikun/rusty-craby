use std::process;
use twilight_command_parser::{Command, CommandParserConfig};
use twilight_http::Client as HttpClient;
use twilight_mention::Mention;
use twilight_model::gateway::payload::MessageCreate;

fn generate_waifu() -> Result<Vec<&'static str>, String> {
    let mut child = process::Command::new("auto-waifu")
        .spawn()
        .expect("auto-waifu command failed to start");
    let _result = child.wait().expect("Failed to wait for command to finish");
    Ok(vec!["avatar.png"])
}

/// Handles the logic of the command
pub async fn handler(
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    generate_waifu().expect("Failed to generate Waifu");

    http.create_message(msg.channel_id)
        .content(format!(
            "{}: Waifu command does not work yet sorry!",
            msg.author.mention()
        ))?
        .await?;
    Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
    command_config.add_command("waifu", true);
    command_config.add_command("w", false);
    command_config.add_command("uwu", false);
    command_config
}

/// Handles matching of if the command has been triggered and calls the handler
pub async fn parse(
    command: &Option<Command<'_>>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command.clone() {
        // Main Command
        Some(Command { name: "waifu", .. }) => handler(msg, http).await,
        // Alias'
        Some(Command { name: "w", .. }) => handler(msg, http).await,
        Some(Command { name: "uwu", .. }) => handler(msg, http).await,
        // Skip anything else
        Some(_) => Ok(()),
        None => Ok(()),
    }
}
