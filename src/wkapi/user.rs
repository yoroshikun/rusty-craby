use reqwest;

#[path = "structs.rs"]
mod structs;

use structs::UserBase;

pub fn get_user(api_token: &str) -> Result<UserBase, reqwest::Error> {
  let client = reqwest::Client::new();
  let mut res = client
    .get("https://api.wanikani.com/v2/user")
    .header("Authorization", format!("Bearer {}", api_token))
    .send()?;
  let user_base: UserBase = res.json()?;
  Ok(user_base)
}
