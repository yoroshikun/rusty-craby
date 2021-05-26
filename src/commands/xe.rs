mod helpers;

use twilight_command_parser::Arguments;
use twilight_command_parser::{Command, CommandParserConfig};
use twilight_embed_builder::EmbedBuilder;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::MessageCreate;

const CURRENCY_CODES: [&str; 66] = [
    "USD", "EUR", "JPY", "BGN", "BTC", "CZK", "DKK", "GBP", "HUF", "PLN", "RON", "SEK", "CHF",
    "ISK", "NOK", "RUB", "TRY", "AUD", "BRL", "CAD", "CNY", "HKD", "IDR", "ILS", "INR", "KRW",
    "MXN", "MYR", "NZD", "PHP", "SGD", "THB", "ZAR", "usd", "eur", "jpy", "bgn", "czk", "dkk",
    "gbp", "huf", "pln", "ron", "sek", "chf", "isk", "nok", "rub", "try", "aud", "brl", "btc",
    "cad", "cny", "hkd", "idr", "ils", "inr", "krw", "mxn", "myr", "nzd", "php", "sgd", "thb",
    "zar",
];

// TODO: Rewrite logic to not copy args
/// Handles the logic of the command
async fn handler(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let count = args.clone().count();
    let arg1 = args.next();
    let arg2 = args.next();
    let arg3 = args.next();

    // skope
    let response = async {
        match count {
            3 => {
                let base = arg1.unwrap_or_default();
                let to = arg2.unwrap_or_default();
                let amount = arg3;

                if base == to {
                    return Err(
                        "Invalid Input, <base> and <to> currencies cannot be the same".to_owned(),
                    );
                }

                if CURRENCY_CODES.contains(&base) == false || CURRENCY_CODES.contains(&to) == false
                {
                    return Err(
                        "Invalid input (currency code), Example: !currency AUD JPY <amount>"
                            .to_owned(),
                    );
                }

                match amount {
                    Some(amount) => match amount.parse::<f64>() {
                        Ok(amount) => {
                            let base = base.to_ascii_uppercase();
                            let to = to.to_ascii_uppercase();

                            let rate = helpers::get_xe_rate(&base, &to).await.unwrap();
                            let response = format!(
                                "{multiplier} {base} --> {to}: **{multiplied_rate:.4}**",
                                base = &base,
                                to = &to,
                                multiplied_rate = rate * amount,
                                multiplier = amount,
                            );

                            Ok(response)
                        }
                        Err(_) => {
                            Err("Invalid input (not float), Example: !currency <amount>".to_owned())
                        }
                    },
                    None => Err("Currency is not a string".to_owned()),
                }
            }
            2 => {
                let base = arg1.unwrap_or_default();
                let to = arg2.unwrap_or_default();

                if base == to {
                    return Err(
                        "Invalid Input, <base> and <to> currencies cannot be the same".to_owned(),
                    );
                }

                if CURRENCY_CODES.contains(&base) == false || CURRENCY_CODES.contains(&to) == false
                {
                    return Err(
                        "Invalid input (currency code), Example: !currency AUD JPY <amount>"
                            .to_owned(),
                    );
                }
                let base = base.to_ascii_uppercase();
                let to = to.to_ascii_uppercase();

                let rate = helpers::get_xe_rate(&base, &to).await.unwrap();

                let response = format!(
                    "{base} --> {to}: **{rate:.4}**",
                    base = &base,
                    to = &to,
                    rate = rate
                );

                Ok(response)
            }
            1 => {
                let amount = arg1.unwrap_or_default();

                match amount.parse::<f64>() {
                    Ok(multiplier) => {
                        let user_id = msg.author.id.0;
                        let user_default = helpers::get_default(user_id);

                        match user_default {
                            Ok(default) => {
                                if "JPY".to_owned() == default {
                                    return Err(
                                    "Invalid Input, <base> and <to> currencies cannot be the same"
                                        .to_owned(),
                                );
                                };

                                let rate = helpers::get_xe_rate(&default, &"JPY".to_owned())
                                    .await
                                    .unwrap();

                                let response = format!(
                                    "{multiplier} {base} --> {to}: **{multiplied_rate:.4}**",
                                    base = &default,
                                    to = "JPY",
                                    multiplied_rate = rate * multiplier,
                                    multiplier = multiplier,
                                );

                                Ok(response)
                            }
                            Err(err) => Err(err),
                        }
                    }
                    Err(_) => {
                        Err("Invalid input (not float), Example: !currency <amount>".to_owned())
                    }
                }
            }
            0 => {
                // Check if user is in the file
                let user_id = msg.author.id.0;
                let user_default = helpers::get_default(user_id);

                println!("{}", user_id);

                match user_default {
                    Ok(default) => {
                        if "JPY".to_owned() == default {
                            return Err(
                                "Invalid Input, <base> and <to> currencies cannot be the same"
                                    .to_owned(),
                            );
                        };

                        let rate = helpers::get_xe_rate(&default, &"JPY".to_owned())
                            .await
                            .unwrap();
                        println!("{}", rate);
                        let response = format!(
                            "{base} --> {to}: **{rate:.4}**",
                            base = &default,
                            to = "JPY",
                            rate = &rate
                        );
                        Ok(response)
                    }
                    Err(err) => Err(err),
                }
            }
            _ => Err("Invalid input, Example: !currency AUD JPY <amount>".to_owned()),
        }
    };

    send_embed(response.await, msg, http).await?;

    Ok(())
}

/// Adds all commands and aliases to the command configuration
pub fn add_commands(mut command_config: CommandParserConfig) -> CommandParserConfig {
    command_config.add_command("xe", true);
    command_config.add_command("xedefault", true);
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
            name: "xe",
            mut arguments,
            ..
        }) => handler(&mut arguments, msg, http).await,
        // Aliases
        Some(Command {
            name: "xedefault",
            mut arguments,
            ..
        }) => handle_set_default(&mut arguments, msg, http).await,
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
            .title("Exchange Rate")?
            .description(description)?
            .color(0xfd_c8_35)?
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
async fn handle_set_default(
    args: &mut Arguments<'_>,
    msg: &MessageCreate,
    http: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let default = match args.next() {
        Some(default) if CURRENCY_CODES.contains(&default) == true => default,
        Some(_) => "AUD",
        None => "AUD",
    };

    let response = helpers::set_default(default, &msg.author.id.0);

    send_embed(response, msg, http).await?;

    Ok(())
}

/// Ensure helper function
pub async fn ensure_all_files() -> Result<(), std::io::Error> {
    helpers::ensure_all_files()
}
