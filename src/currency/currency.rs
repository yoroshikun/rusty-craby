use serde::Deserialize;
use serde_json::Value;

use serenity::model::channel::Message;

#[path = "helpers.rs"]
mod helpers;

#[derive(Deserialize, Debug)]
struct Exchange {
  rates: Value,
  base: String,
  date: String,
}

static CURRENCY_CODES: [&str; 64] = [
  "USD", "EUR", "JPY", "BGN", "CZK", "DKK", "GBP", "HUF", "PLN", "RON", "SEK", "CHF", "ISK", "NOK",
  "RUB", "TRY", "AUD", "BRL", "CAD", "CNY", "HKD", "IDR", "ILS", "INR", "KRW", "MXN", "MYR", "NZD",
  "PHP", "SGD", "THB", "ZAR", "usd", "eur", "jpy", "bgn", "czk", "dkk", "gbp", "huf", "pln", "ron",
  "sek", "chf", "isk", "nok", "rub", "try", "aud", "brl", "cad", "cny", "hkd", "idr", "ils", "inr",
  "krw", "mxn", "myr", "nzd", "php", "sgd", "thb", "zar",
];

// Get Current Currencty
fn get_current_exchange<S: Into<String>>(base: S, to: S) -> Result<f64, reqwest::Error> {
  // Perform the conversion to String
  let base = base.into();
  let to = to.into();

  // Form and send request
  let request_url = format!(
    "https://api.exchangeratesapi.io/latest?base={}&symbols={}",
    &base, &to
  );
  let mut response = reqwest::get(&request_url)?;

  // Deserialize result into semi typed struct
  let rates: Exchange = response.json()?;

  // Print debug if in debug mode
  if cfg!(debug_assertions) {
    println!("{:?}", rates.rates);
  }

  let rate = rates.rates[&to]
    .as_f64()
    .expect("Unable to convert rate to float");

  Ok(rate)
}

pub fn handler(msg: &Message) -> Result<String, String> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();
  // Simple check to ensure two arguments were given
  let response = match content_chunks.len() {
    4 => {
      let param1 = content_chunks[1];
      let param2 = content_chunks[2];
      let param3 = content_chunks[3];

      if param1 == param2 {
        return Err("Invalid Input, <base> and <to> currencies cannot be the same".to_owned());
      };

      match param3.parse::<f64>() {
        Ok(multiplier) => {
          let base = param1.to_ascii_uppercase();
          let to = param2.to_ascii_uppercase();

          let rate = get_current_exchange(&base, &to).unwrap();
          let response = format!(
            "{multiplier} {base} --> {to}: **{multiplied_rate:.4}**",
            base = &base,
            to = &to,
            multiplied_rate = rate * multiplier,
            multiplier = multiplier,
          );

          Ok(response)
        }
        Err(_) => Err("Invalid input (not float), Example: !currency <amount>".to_owned()),
      }
    }
    3 => {
      let param1 = content_chunks[1];
      let param2 = content_chunks[2];

      if param1 == param2 {
        return Err("Invalid Input, <base> and <to> currencies cannot be the same".to_owned());
      };

      // Ensure the from and to are valid
      if CURRENCY_CODES.contains(&param1) && CURRENCY_CODES.contains(&param2) {
        let base = param1.to_ascii_uppercase();
        let to = param2.to_ascii_uppercase();
        let rate = get_current_exchange(&base, &to).unwrap();
        let response = format!(
          "{base} --> {to}: **{rate:.4}**",
          base = &base,
          to = &to,
          rate = rate
        );
        Ok(response)
      } else {
        // Secondary check if param1 is keyword "default"
        match param1 {
          "default" => {
            if CURRENCY_CODES.contains(&param2) {
              let set_default = helpers::set_default(param2, msg.author.id.as_u64());
              match set_default {
                Ok(response) => Ok(response),
                Err(err) => Err(err),
              }
            } else {
              Err("Invalid input, Example !currency default USD".to_owned())
            }
          }
          _ => Err("Invalid input, Example: !currency AUD JPY <amount>".to_owned()),
        }
      }
    }
    2 => {
      let param1 = content_chunks[1];

      match param1.parse::<f64>() {
        Ok(multiplier) => {
          let user_default = helpers::get_default(msg.author.id.as_u64());
          match user_default {
            Ok(default) => {
              if "JPY".to_owned() == default {
                return Err(
                  "Invalid Input, <base> and <to> currencies cannot be the same".to_owned(),
                );
              };
              let rate = get_current_exchange(&default, &"JPY".to_owned()).unwrap();
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
    1 => {
      // Check if user is in the file
      let user_default = helpers::get_default(msg.author.id.as_u64());

      match user_default {
        Ok(default) => {
          if "JPY".to_owned() == default {
            return Err("Invalid Input, <base> and <to> currencies cannot be the same".to_owned());
          };

          let rate = get_current_exchange(&default, &"JPY".to_owned()).unwrap();
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
