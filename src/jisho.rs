use serde::Deserialize;
use serde_json::Value;

use serenity::model::channel::Message;

#[derive(Deserialize, Debug)]
struct Jisho {
  meta: JishoMeta,
  data: Option<Vec<JishoDataItem>>,
}

#[derive(Deserialize, Debug)]
struct JishoMeta {
  status: u32,
}

#[derive(Deserialize, Debug)]
struct JishoDataItem {
  slug: String,
  is_common: bool,
  tags: Vec<String>,
  jlpt: Vec<String>,
  japanese: Vec<JishoDataJapanese>,
  senses: Vec<JishoDataSenses>,
  attribution: JishoDataAttribution,
}

#[derive(Deserialize, Debug)]
struct JishoDataJapanese {
  word: Option<String>,
  reading: String,
}

#[derive(Deserialize, Debug)]
struct JishoDataSenses {
  english_definitions: Vec<String>,
  parts_of_speech: Vec<String>,
  links: Vec<JishoDataLinks>,
  tags: Vec<String>,
  restrictions: Vec<String>,
  see_also: Vec<String>,
  antonyms: Vec<String>,
  source: Vec<JishoDataSource>,
  info: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct JishoDataAttribution {
  jmdict: Value,
  jmnedict: Value,
  dbpedia: Value,
}

#[derive(Deserialize, Debug)]
struct JishoDataLinks {
  text: String,
  url: String,
}

#[derive(Deserialize, Debug)]
struct JishoDataSource {
  language: String,
  word: String,
}

/// Search Jisho for given character or string
fn get_jisho(word: &str) -> Result<(String, String), reqwest::Error> {
  // Form and send request
  let request_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);
  let mut response = reqwest::get(&request_url)?;

  // Deserialize result into semi typed struct
  let jisho: Jisho = response.json()?;

  let description = match &jisho.data {
    Some(data) => {
      if data.len() == 0 {
        "No Jisho Results".to_owned()
      } else {
        format!(
          "**English**: {} \n **Japanese**: {} \n **Reading**: {} \n ---------- \n More information: {}",
          data[0].senses[0].english_definitions[0],
          data[0].japanese[0]
            .word
            .as_ref()
            .unwrap_or(&"No Kana".to_owned()),
          data[0].japanese[0].reading,
          format!("https://jisho.org/word/{}", data[0].slug)
        )
      }
    }
    None => "No Jisho Results".to_owned(),
  };

  let url = match &jisho.data {
    Some(data) => format!("https://jisho.org/word/{}", data[0].slug),
    None => "".to_owned(),
  };

  Ok((description, url))
}

pub fn handler(msg: &Message) -> Result<(String, String), String> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();

  // Simple check to ensure word is given
  let response = match content_chunks.len() {
    2 => {
      let word = content_chunks[1];
      get_jisho(word).unwrap()
    }
    _ => return Err("The input is invalid, Example: !jisho person".to_owned()),
  };

  Ok(response)
}
