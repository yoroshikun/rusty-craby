mod helpers;

use twilight_command_parser::Arguments;
use twilight_command_parser::{Command, CommandParserConfig};
use twilight_embed_builder::EmbedBuilder;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::MessageCreate;

/// Handles the logic of the command
pub async fn handler(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = match args.next() {
        Some(word) => helpers::format_jisho(word).await,
        None => Err("The input is invalid, Example: !jisho person".to_owned()),
    };

    let emebed = match response {
        Ok((description, url)) => EmbedBuilder::new()
            .description(description)?
            .url(url)
            .color(0x00_fa_9A)?
            .build(),
        Err(err) => EmbedBuilder::new()
            .description(err)?
            .color(0xfd_35_35)?
            .build(),
    };

    http.create_message(msg.channel_id)
        .embed(emebed.unwrap())?
        .await?;

    Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
    command_config.add_command("jisho", true);
    command_config.add_command("j", false);
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
        Some(Command {
            name: "jisho",
            mut arguments,
            ..
        }) => handler(&mut arguments, msg, http).await,
        // Aliases
        Some(Command {
            name: "j",
            mut arguments,
            ..
        }) => handler(&mut arguments, msg, http).await,
        // Skip anything else
        Some(_) => Ok(()),
        None => Ok(()),
    }
}
