use twilight_command_parser::{CommandParserConfig, Parser};

pub mod help;
pub mod jisho;
pub mod ping;
pub mod tubby;
pub mod waifu;
pub mod xe;

/// Creates and returns a new parser with all commands added
pub fn create_parser<'a>() -> Parser<'a> {
    let mut command_config = CommandParserConfig::new();

    command_config = ping::add_commands(command_config);
    command_config = help::add_commands(command_config);
    command_config = jisho::add_commands(command_config);
    command_config = tubby::add_commands(command_config);
    command_config = xe::add_commands(command_config);
    command_config = waifu::add_commands(command_config);

    command_config.add_prefix("!");

    Parser::new(command_config)
}
