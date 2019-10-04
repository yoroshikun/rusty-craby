extern crate serde_yaml;
extern crate serenity;

use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::{Context, EventHandler};

mod currency;
mod wk_levels;
mod wkapi;

group!({
    name: "general",
    options: {},
    commands: [ping, currency],
});

group!({
    name: "wanikani",
    options: {},
    commands: [levels, add_wkapi]
});

use std::env;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Find openssl certs
    openssl_probe::init_ssl_cert_env_vars();
    // Log in to Discord using a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Expected token"), Handler)
        .expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
            .group(&GENERAL_GROUP)
            .group(&WANIKANI_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(err) = client.start() {
        println!("An error occurred while running the client: {:?}", err);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
#[aliases("xe")]
fn currency(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = currency::handler(msg);
    msg.channel_id.say(&ctx.http, response)?;

    Ok(())
}

#[command]
fn levels(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = wk_levels::handler(msg);
    msg.channel_id.say(&ctx.http, response)?;

    Ok(())
}

#[command]
fn add_wkapi(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = wkapi::api_tokens::add_api_token(msg).expect("Failed to add api_token");
    msg.channel_id.say(&ctx.http, response)?;

    Ok(())
}
