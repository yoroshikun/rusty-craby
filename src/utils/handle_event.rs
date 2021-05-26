use std::error::Error;

use tokio::join;

use twilight_command_parser::Parser;
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;

use crate::commands::{help, jisho, ping, tubby, waifu, xe};

pub async fn handle_event(
    shard_id: u64,
    event: Event,
    http: HttpClient,
    parser: Parser<'_>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) => {
            let parsed = parser.parse(&msg.content);

            let ping_parse = ping::parse(&parsed, &msg, &http);
            let help_parse = help::parse(&parsed, &msg, &http);
            let jisho_parse = jisho::parse(&parsed, &msg, &http);
            let tubby_parse = tubby::parse(&parsed, &msg, &http);
            let xe_parse = xe::parse(&parsed, &msg, &http);
            let waifu_parse = waifu::parse(&parsed, &msg, &http);

            // Run all at the same time and return once all done
            join!(
                ping_parse,
                help_parse,
                jisho_parse,
                tubby_parse,
                xe_parse,
                waifu_parse
            );
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        _ => {}
    }

    Ok(())
}
