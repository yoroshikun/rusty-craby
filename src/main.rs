mod commands;
mod utils;

use futures::stream::StreamExt;
use std::{env, error::Error};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Intents,
};
use twilight_http::Client as HttpClient;
use utils::handle_event::handle_event;

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

    let parser = commands::create_parser();

    // Ensure file structure (to be moved)
    commands::xe::ensure_all_files()
        .await
        .expect("Failed to ensure required files: XE");

    commands::tubby::ensure_all_files()
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

        // Spawn a new task to handle the event (clone http and parser for lifetimes in async)
        tokio::spawn(handle_event(shard_id, event, http.clone(), parser.clone()));
    }

    Ok(())
}
