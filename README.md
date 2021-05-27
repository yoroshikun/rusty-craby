# Rusty Craby Twilight

Rusty Craby is a Discord bot primarly built for the WKClass Citizens (Discord Server).

The bot is built with the [twilight](https://github.com/twilight-rs/twilight) crate which has been adapted to be as readable as possible when it comes to command logic.

Feel free to contibute to the project if you can improve my rust code, as this project is mainly used as a learning resource.

## Commands

| Command | Description                                                         |
| ------- | ------------------------------------------------------------------- |
| Jisho   | Fetch a word using the jisho dictonary api                          |
| Tubby   | A helper for managing tubby requests for Genshin Impact             |
| XE      | Quick currency exchange rates                                       |
| Waifu   | Generate a waifu using the [Waifulabs](https://waifulabs.com/) site |
| Ping    | Simple ping command! You know the drill                             |
| Help    | Help command that explains the rest                                 |

## Setup

This bot requires rust to compile and run, the easiest way to get this setup is though rustup and cargo.

1. Install rustup and cargo -> [Rustup Site](https://rustup.rs/)
2. Fill in .envrc, See Testing below (I use [direnv](https://github.com/direnv/direnv) to manage env variables easily)
3. Build and run Debug `cargo run`
4. Build for release with `cargo build --release` with the executable located at `./target/release/rusty-craby`

## Testing

For testing your builds locally you can use a test bot.

1. Head to the [Discord Developer Portal](https://discordapp.com/developers/) and create a new app. Name can be whatever you want. I named mine Rusty Craby Test
2. Under the Bot section create your bot and copy the token.
3. Add your token to the .envrc `TEST_DISCORD_TOKEN`
4. You can now add your bot to a server, under the oatuh2 section. under scopes select bot then administrator then copy the generated url to a new browser tab to authroize and connect it to a test server
5. Building with cargo in debug mode (without the --release option) will automatically use the `TEST_DISCORD_TOKEN` over the `DISCORD_TOKEN`.
6. Building with cargo in release mode will use the `DISCORD_TOKEN` instead

## Cross Compiling

TBA

For now the programm must be compiled on device eg. ARM
