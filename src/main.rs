extern crate serde_yaml;
extern crate serenity;

use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::{Context, EventHandler};

use serenity::utils::Colour;

mod currency;
mod jisho;
mod wk_levels;
mod wkapi;

group!({
    name: "general",
    options: {},
    commands: [ping, currency, jisho],
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

    // If debug mode use test token (fallback default discord token) / Panic if discord token is not a thing
    let token = if cfg!(debug_assertions) {
        match env::var("TEST_DISCORD_TOKEN") {
            Ok(token) => token,
            Err(_) => env::var("DISCORD_TOKEN").expect("Expected Token"),
        }
    } else {
        env::var("DISCORD_TOKEN").expect("Expected Token")
    };

    // Log in to Discord using a bot token from the environment
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
            .group(&GENERAL_GROUP)
            .group(&WANIKANI_GROUP),
    );

    // Ensure all required files are created on launch
    wkapi::ensure::ensure_all().expect("Failed to ensure required files");

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

    match response {
        Ok(response) => msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e.title("Exchange Rate");
                e.description(response);
                e.colour(Colour::from_rgb(253, 216, 53));

                e
            })
        })?,
        Err(err) => msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e.title("Exchange Rate");
                e.description(err);
                e.colour(Colour::from_rgb(255, 23, 68));

                e
            })
        })?,
    };

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

#[command]
#[aliases("J", "j")]
fn jisho(ctx: &mut Context, msg: &Message) -> CommandResult {
    let response = jisho::handler(msg);

    match response {
        Ok((description, url)) => msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e.description(description);
                e.url(url);
                e.colour(Colour::from_rgb(0, 250, 154));

                e
            })
        })?,
        Err(err) => msg.channel_id.say(&ctx.http, err)?,
    };

    Ok(())
}
