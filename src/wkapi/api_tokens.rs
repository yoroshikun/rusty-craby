use serde::{Deserialize, Serialize};
use serde_yaml;

use std::fs::File;
use std::fs::OpenOptions;

use serenity::model::channel::Message;

#[path = "user.rs"]
mod user;

use user::get_user;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiTokens {
  pub tokens: Vec<UserTokens>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTokens {
  pub id: String,
  pub token: String,
}

pub fn get_api_token(id: String) -> Result<String, serde_yaml::Error> {
  let api_tokens: ApiTokens = serde_yaml::from_reader(
    OpenOptions::new()
      .write(true)
      .read(true)
      .create(true)
      .open("wkdata/api_tokens.yaml")
      .expect("Failed to open File"),
  )?;
  let user = api_tokens
    .tokens
    .iter()
    .find(|&r| r.id == id)
    .expect("Could not find user");

  Ok(user.token.to_owned())
}

pub fn add_api_token(msg: &Message) -> Result<String, serde_yaml::Error> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();

  // Match for exactly 2 args (if anything else invalid)
  let response = match content_chunks.len() {
    2 => {
      let api_token = content_chunks[1];
      // Ensure apitoken is valid
      let user = get_user(api_token.to_owned()).expect("User token was invalid");
      // Read old file
      let api_tokens: ApiTokens = serde_yaml::from_reader(
        OpenOptions::new()
          .write(true)
          .read(true)
          .create(true)
          .open("wkdata/api_tokens.yaml")
          .expect("Failed to open File"),
      )?;
      // New user tokens vec
      let mut new_user_tokens: Vec<UserTokens> = api_tokens.tokens;
      // Find and remove the current value if value already exists
      let removed = new_user_tokens
        .iter()
        .position(|token| *token.token == api_token.to_owned())
        .map(|e| new_user_tokens.remove(e))
        .is_some();
      println!("Removed existing: {}", removed);
      // Add to end of users
      new_user_tokens.push(UserTokens {
        id: user.data.id,
        token: api_token.to_owned(),
      });
      // Write new yaml file
      let new_api_tokens = ApiTokens {
        tokens: new_user_tokens,
      };
      let buffer = File::create("wkdata/api_tokens.yaml").expect("Failed to create file");
      serde_yaml::to_writer(buffer, &new_api_tokens).expect("Failed to write yaml");
      // Send confirmation
      format!("Successfully added new token: {}", api_token.to_owned())
    }
    _ => "The input is invalid, Example: !add_wkapi <api_token>".to_owned(),
  };
  Ok(response)
}
