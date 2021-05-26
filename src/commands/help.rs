use twilight_command_parser::{Arguments, Command, CommandParserConfig};
use twilight_embed_builder::EmbedBuilder;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::MessageCreate;

fn describe_command(word: &str) -> Result<String, String> {
  match word {
      "jisho" => {
        Ok("Search for a specified word or phrase \n\n**Usage** !jisho <word> \n**Example** !jisho hello \n**Alias**: j, J".to_owned())
      }
      "xe" => {
        Ok("Check current exchange rates \n\n**Usage** !xe <from> <to> <amount> \n**Example** !xe AUD JPY 800 \n**Alias**: xe \n**Set Default** !exdefault AUD \n**Shorthand (uses default)** !xe or !xe <from> \n to currency is always JPY".to_owned())
      }
      "tubby" => {
        Ok("Organize Genshin Tubby requests \n\n**Usage** !tubby <create | complete>? <user>?\n**Alias**: t".to_owned())
      }
      "waifu" => {
        Ok("Generate a waifu with AI \n\n**Usage** !waifu \n**Alias**: w, uwu".to_owned())
      }
    //   "levels" => {
    //     Ok("List current levels of registered wkapi users \n\n**Usage** !levels \n\n**Alpha**".to_owned())
    //   }
    //   "add_wkapi" => {
    //     Ok("Add a new api key to the system \n\n**Usage** !add_api <api_key> \n\n**Important** Use api key v2 \n\n**Alpha**".to_owned())
    //   }
      _ => Err("No command found with that name".to_owned()),
    }
}

/// Handles the logic of the command
pub async fn handler(
  args: &mut Arguments<'_>,
  msg: &MessageCreate,
  http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let response = match args.next() {
      Some(command) => describe_command(command),
      None => {
        Ok("List of available commands \n**jisho**: Search Jisho for a word or phrase \n**xe**: Check current exchange rates \n **tubby**: Organize Genshin Tubby requests \n**waifu**: Generate a waifu with AI".to_owned())
      }
    };

  let emebed = match response {
    Ok(response) => EmbedBuilder::new()
      .description(response)?
      .color(0x78_90_9c)?
      .build(),
    Err(err) => EmbedBuilder::new()
      .description(err)?
      .color(0xfd_35_35)?
      .build(),
  };

  http
    .create_message(msg.channel_id)
    .embed(emebed.unwrap())?
    .await?;

  Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
  command_config.add_command("help", true);
  command_config.add_command("h", false);
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
      name: "help",
      mut arguments,
      ..
    }) => handler(&mut arguments, msg, http).await,
    // Alias'
    Some(Command {
      name: "h",
      mut arguments,
      ..
    }) => handler(&mut arguments, msg, http).await,
    // Skip anything else
    Some(_) => Ok(()),
    None => Ok(()),
  }
}
