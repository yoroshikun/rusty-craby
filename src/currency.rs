use serde::Deserialize;
use serde_json::Value;

use serenity::model::channel::Message;

#[derive(Deserialize, Debug)]
struct Exchange {
  rates: Value,
  base: String,
  date: String,
}

static CURRENCY_CODES: [&str; 31] = [
  "USD", "JPY", "BGN", "CZK", "DKK", "GBP", "HUF", "PLN", "RON", "SEK", "CHF", "ISK", "NOK", "RUB",
  "TRY", "AUD", "BRL", "CAD", "CNY", "HKD", "IDR", "ILS", "INR", "KRW", "MXN", "MYR", "NZD", "PHP",
  "SGD", "THB", "ZAR",
];

// Get Current Currencty
fn get_current_exchange(base: &str, to: &str) -> Result<String, reqwest::Error> {
  // Form and send request
  let request_url = format!(
    "https://api.exchangeratesapi.io/latest?base={}&symbols={}",
    base, to
  );
  let mut response = reqwest::get(&request_url)?;

  // Deserialize result into semi typed struct
  let rates: Exchange = response.json()?;

  // Print debug if in debug mode
  if cfg!(debug_assertions) {
    println!("{:?}", rates.rates);
  }
  // Convert the requested rate to a string
  let rate = format!(
    "{base} --> {to}: **{rate:.4}**",
    base = base,
    to = to,
    rate = rates.rates[to]
      .as_f64()
      .expect("Could not parse rate into float")
  );
  Ok(rate)
}

pub fn handler(msg: &Message) -> Result<String, String> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();
  // Simple check to ensure two arguments were given
  let response = match content_chunks.len() {
    3 => {
      let base = content_chunks[1];
      let to = content_chunks[2];
      // Ensure the from and to are valid
      if CURRENCY_CODES.contains(&base) && CURRENCY_CODES.contains(&to) {
        Ok(get_current_exchange(base, to).unwrap())
      } else {
        Err("Invalid input, Example: !currency AUD JPY".to_owned())
      }
    }
    2 => Err("Invalid input, Example: !currency AUD JPY".to_owned()),
    1 => Ok(get_current_exchange("AUD", "JPY").unwrap()),
    _ => Err("Invalid input, Example: !currency AUD JPY".to_owned()),
  };
  response
}
