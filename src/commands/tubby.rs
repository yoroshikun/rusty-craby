mod helpers;

use twilight_command_parser::Arguments;
use twilight_command_parser::{Command, CommandParserConfig};
use twilight_embed_builder::EmbedBuilder;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::MessageCreate;

/// Handles the logic of the command
async fn handler(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = match args.next() {
        Some(subcommand) => match subcommand.to_ascii_lowercase().as_str() {
            "request" => {
                let offset = match args.next() {
                    Some(offset) => match offset.parse::<u8>() {
                        Ok(offset) => Some(offset),
                        Err(_) => None,
                    },
                    None => None,
                };

                helpers::create_request(&msg.author, offset)
            }
            "create" => {
                let offset = match args.next() {
                    Some(offset) => match offset.parse::<u8>() {
                        Ok(offset) => Some(offset),
                        Err(_) => None,
                    },
                    None => None,
                };

                helpers::create_request(&msg.author, offset)
            }
            "complete" => match args.next() {
                Some(complete_user) => helpers::complete_request(complete_user),
                None => Err("A user must be provided".to_owned()),
            },
            "list " => match helpers::get_requests() {
                Ok(requests) => Ok(requests.join("\n")),
                Err(err) => Err(err),
            },
            _ => match helpers::get_requests() {
                Ok(requests) => Ok(requests.join("\n")),
                Err(err) => Err(err),
            },
        },
        None => match helpers::get_requests() {
            Ok(requests) => Ok(requests.join("\n")),
            Err(err) => Err(err),
        },
    };

    send_embed(response, msg, http).await?;

    Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
    command_config.add_command("tubby", true);
    command_config.add_command("t", false);
    command_config.add_command("tc", false);
    command_config.add_command("tr", false);
    command_config.add_command("tl", false);
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
            name: "tubby",
            mut arguments,
            ..
        }) => handler(&mut arguments, msg, http).await,
        // Aliases
        Some(Command {
            name: "t",
            mut arguments,
            ..
        }) => handler(&mut arguments, msg, http).await,
        Some(Command {
            name: "tc",
            mut arguments,
            ..
        }) => handle_complete_request(&mut arguments, msg, http).await,
        Some(Command {
            name: "tr",
            mut arguments,
            ..
        }) => handle_create_request(&mut arguments, msg, http).await,
        Some(Command { name: "tl", .. }) => handle_list_requests(msg, http).await,
        // Skip anything else
        Some(_) => Ok(()),
        None => Ok(()),
    }
}

/// Utility function for responding with embed
async fn send_embed(
    response: Result<String, String>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let emebed = match response {
        Ok(description) => EmbedBuilder::new()
            .title("Tubby Manager")?
            .description(description)?
            .color(0x96_37_B3)?
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

/// Shortcut handler for creating a request
async fn handle_create_request(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let offset = match args.next() {
        Some(offset) => match offset.parse::<u8>() {
            Ok(offset) => Some(offset),
            Err(_) => None,
        },
        None => None,
    };

    let response = helpers::create_request(&msg.author, offset);

    send_embed(response, msg, http).await?;

    Ok(())
}

/// Shortcut handler for completing a request
async fn handle_complete_request(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = match args.next() {
        Some(complete_user) => helpers::complete_request(complete_user),
        None => Err("A user must be provided".to_owned()),
    };

    send_embed(response, msg, http).await?;

    Ok(())
}
/// Shortcut handler for getting requests
async fn handle_list_requests(
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = match helpers::get_requests() {
        Ok(requests) => Ok(requests.join("\n")),
        Err(err) => Err(err),
    };

    send_embed(response, msg, http).await?;

    Ok(())
}

/// Ensure helper function
pub async fn ensure_all_files() -> Result<(), std::io::Error> {
    helpers::ensure_all_files()
}
