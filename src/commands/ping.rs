use twilight_command_parser::{Command, CommandParserConfig};
use twilight_http::Client as HttpClient;
use twilight_mention::Mention;
use twilight_model::gateway::payload::MessageCreate;

/// Handles the logic of the command
pub async fn handler(
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    http.create_message(msg.channel_id)
        .content(format!("{}: Pong!", msg.author.mention()))?
        .await?;
    Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
    command_config.add_command("ping", true);
    command_config.add_command("p", false);
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
        Some(Command { name: "ping", .. }) => handler(msg, http).await,
        // Alias'
        Some(Command { name: "p", .. }) => handler(msg, http).await,
        // Skip anything else
        Some(_) => Ok(()),
        None => Ok(()),
    }
}
