use futures::stream::StreamExt;
use std::{env, error::Error};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_command_parser::{Command, CommandParserConfig, Parser};
use twilight_embed_builder::{EmbedBuilder, ImageSource};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event, Intents,
};
use twilight_http::Client as HttpClient;
use twilight_mention::Mention;

mod help;
mod jisho;
mod tubby;
mod waifu;
mod xe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // If debug mode use test token (fallback default discord token) / Panic if discord token is not a thing
    let token = if cfg!(debug_assertions) {
        match env::var("TEST_DISCORD_TOKEN") {
            Ok(token) => token,
            Err(_) => env::var("DISCORD_TOKEN").expect("Expected Test Token"),
        }
    } else {
        env::var("DISCORD_TOKEN").expect("Expected Token")
    };

    // Default sharding
    let scheme = ShardScheme::Auto;

    // Specify intents requesting events about things like new and updated
    // messages in a guild and direct messages.
    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES;

    let cluster = Cluster::builder(&token, intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    // Start up the cluster
    let cluster_spawn = cluster.clone();

    // Command Setup
    let mut command_config = CommandParserConfig::new();

    // (Use `Config::add_command` to add a single command)
    command_config.add_command("ping", true);
    command_config.add_command("help", true);
    command_config.add_command("h", false);
    // XE
    command_config.add_command("xe", true);
    command_config.add_command("xedefault", true);
    // JISHO
    command_config.add_command("jisho", true);
    command_config.add_command("j", false);
    // TUBBY
    command_config.add_command("tubby", true);
    command_config.add_command("t", false);
    command_config.add_command("tl", false);
    command_config.add_command("tr", false);
    command_config.add_command("tc", false);
    // WAIFU
    command_config.add_command("waifu", true);
    command_config.add_command("w", false);
    command_config.add_command("uwu", false);

    // Add the prefix `"!"`.
    // (Use `Config::add_prefixes` to add multiple prefixes)
    command_config.add_prefix("!");

    let parser = Parser::new(command_config);

    // Ensure file structure
    xe::ensure_all_files()
        .await
        .expect("Failed to ensure required files: XE");

    tubby::ensure_all_files()
        .await
        .expect("Failed to ensure required files: Tubby");

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // The http client is seperate from the gateway,
    // so startup a new one
    let http = HttpClient::new(&token);

    // Since we only care about messages, make the cache only process messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    let mut events = cluster.events();

    // Startup an event loop to process each event in the event stream as they
    // come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache.
        cache.update(&event);

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(shard_id, event, http.clone(), parser.clone()));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: HttpClient,
    parser: Parser<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            match parser.parse(&msg.content) {
                Some(Command {
                    name: "xe",
                    arguments,
                    ..
                }) => {
                    let response = xe::get_xe(arguments, msg.author.id).await;

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Exchange Rate")?
                            .description(response)?
                            .color(0xfd_c8_35)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Exchange Rate")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "xedefault",
                    mut arguments,
                    ..
                }) => {
                    let default = arguments.next();

                    if default.is_some() {
                        let response = xe::set_default(default.unwrap(), &msg.author.id.0).await;

                        let emebed = match response {
                            Ok(response) => EmbedBuilder::new()
                                .title("Exchange Rate")?
                                .description(response)?
                                .color(0xfd_c8_35)?
                                .build(),
                            Err(err) => EmbedBuilder::new()
                                .title("Exchange Rate")?
                                .description(err)?
                                .color(0xfd_35_35)?
                                .build(),
                        };
                        http.create_message(msg.channel_id)
                            .embed(emebed.unwrap())?
                            .await?;
                    }
                }
                Some(Command {
                    name: "jisho",
                    arguments,
                    ..
                }) => {
                    let response = jisho::handler(arguments).await;

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
                }
                Some(Command {
                    name: "j",
                    arguments,
                    ..
                }) => {
                    let response = jisho::handler(arguments).await;

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
                }
                Some(Command {
                    name: "help",
                    arguments,
                    ..
                }) => {
                    let response = help::handler(arguments);

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

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "h",
                    arguments,
                    ..
                }) => {
                    let response = help::handler(arguments);

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

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "tubby",
                    arguments,
                    ..
                }) => {
                    let user = msg.author.clone();
                    let response = tubby::handler(arguments, user);

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(response)?
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "t",
                    arguments,
                    ..
                }) => {
                    let user = msg.author.clone();
                    let response = tubby::handler(arguments, user);

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(response)?
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "tr",
                    mut arguments,
                    ..
                }) => {
                    let user = msg.author.clone();
                    let offset = match arguments.next() {
                        Some(arg) => match arg.parse::<u8>() {
                            Ok(offset) => Some(offset),
                            Err(_) => None,
                        },
                        None => None,
                    };
                    let response = tubby::create_request(user, offset);

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(response)?
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command {
                    name: "tc",
                    mut arguments,
                    ..
                }) => {
                    let user = arguments.next().unwrap();
                    let response = tubby::complete_request(user);

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(response)?
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command { name: "tl", .. }) => {
                    let response = tubby::get_requests();

                    let emebed = match response {
                        Ok(response) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(response)?
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .title("Tubby Manager")?
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command { name: "waifu", .. }) => {
                    let response = waifu::handler();

                    let emebed = match response {
                        Ok(description) => EmbedBuilder::new()
                            .description(description)?
                            .image(ImageSource::attachment("~/auto-waifu/avatar.png")?)
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command { name: "w", .. }) => {
                    let response = waifu::handler();

                    let emebed = match response {
                        Ok(description) => EmbedBuilder::new()
                            .description(description)?
                            .image(ImageSource::attachment("~/auto-waifu/avatar.png")?)
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command { name: "uwu", .. }) => {
                    let response = waifu::handler();

                    let emebed = match response {
                        Ok(description) => EmbedBuilder::new()
                            .description(description)?
                            .image(ImageSource::attachment("~/auto-waifu/avatar.png")?)
                            .color(0xc4_46_e0)?
                            .build(),
                        Err(err) => EmbedBuilder::new()
                            .description(err)?
                            .color(0xfd_35_35)?
                            .build(),
                    };

                    http.create_message(msg.channel_id)
                        .embed(emebed.unwrap())?
                        .await?;
                }
                Some(Command { name: "ping", .. }) => {
                    http.create_message(msg.channel_id)
                        .content(format!("{}: Pong!", msg.author.mention()))?
                        .await?;
                }
                // Ignore all other commands.
                Some(_) => {}
                None => {}
            }
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        _ => {}
    }

    Ok(())
}
