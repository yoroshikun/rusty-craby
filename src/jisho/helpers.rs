#[path = "structs.rs"]
mod structs;

use structs::Jisho;

async fn get_jisho(word: &str) -> Result<Jisho, reqwest::Error> {
    // Form and send request
    let request_url = format!("https://jisho.org/api/v1/search/words?keyword={}", word);
    let jisho: Jisho = reqwest::get(&request_url).await?.json().await?;

    Ok(jisho)
}

pub async fn format_jisho(word: &str) -> Result<(String, String), String> {
    let jisho = match get_jisho(word).await {
        Ok(jisho) => jisho,
        Err(_) => return Err("Failed to get jisho results from api".to_owned()),
    };
    match &jisho.data {
        Some(data) => {
            if data.len() == 0 {
                Err("No Jisho Results".to_owned())
            } else {
                let response = format!(
            "**English**: {} \n**Japanese**: {} \n**Reading**: {} \n----------\n[\u{1F4D7}]({}) | [\u{1F50D}]({})",
            data[0].senses[0].english_definitions[0],
            data[0].japanese[0]
              .word
              .as_ref()
              .unwrap_or(&"No Kana".to_owned()),
            data[0].japanese[0].reading.as_ref().unwrap_or(&"No reading".to_owned()),
            format!("https://jisho.org/word/{}", data[0].slug),
            format!("https://jisho.org/search/{}", word)
          );
                let url = format!("https://jisho.org/search/{}", word);

                Ok((response, url))
            }
        }
        None => Err("No Jisho Results".to_owned()),
    }
}
