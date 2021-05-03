use twilight_command_parser::Arguments;
use twilight_model::id::UserId;

mod helpers;
mod structs;

const CURRENCY_CODES: [&str; 66] = [
    "USD", "EUR", "JPY", "BGN", "BTC", "CZK", "DKK", "GBP", "HUF", "PLN", "RON", "SEK", "CHF",
    "ISK", "NOK", "RUB", "TRY", "AUD", "BRL", "CAD", "CNY", "HKD", "IDR", "ILS", "INR", "KRW",
    "MXN", "MYR", "NZD", "PHP", "SGD", "THB", "ZAR", "usd", "eur", "jpy", "bgn", "czk", "dkk",
    "gbp", "huf", "pln", "ron", "sek", "chf", "isk", "nok", "rub", "try", "aud", "brl", "btc",
    "cad", "cny", "hkd", "idr", "ils", "inr", "krw", "mxn", "myr", "nzd", "php", "sgd", "thb",
    "zar",
];

pub async fn get_xe(mut arguments: Arguments<'_>, id: UserId) -> Result<String, String> {
    let count = arguments.clone().count();
    let arg1 = arguments.next();
    let arg2 = arguments.next();
    let arg3 = arguments.next();

    let response = match count {
        3 => {
            let base = arg1.unwrap_or_default();
            let to = arg2.unwrap_or_default();
            let amount = arg3;

            if base == to {
                return Err(
                    "Invalid Input, <base> and <to> currencies cannot be the same".to_owned(),
                );
            };

            if CURRENCY_CODES.contains(&base) == false || CURRENCY_CODES.contains(&to) == false {
                return Err(
                    "Invalid input (currency code), Example: !currency AUD JPY <amount>".to_owned(),
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
                None => {
                    return Err("Currency is not a string".to_owned());
                }
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

            if CURRENCY_CODES.contains(&base) == false || CURRENCY_CODES.contains(&to) == false {
                return Err(
                    "Invalid input (currency code), Example: !currency AUD JPY <amount>".to_owned(),
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
                    let user_id = id.0;
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
                Err(_) => Err("Invalid input (not float), Example: !currency <amount>".to_owned()),
            }
        }
        0 => {
            // Check if user is in the file
            let user_id = id.0;
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
    };
    response
}

pub async fn set_default(default: &str, id: &u64) -> Result<String, String> {
    helpers::set_default(default, id)
}

pub async fn ensure_all_files() -> Result<(), std::io::Error> {
    helpers::ensure_all_files()
}
