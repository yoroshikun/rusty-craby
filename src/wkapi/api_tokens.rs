use serde_yaml;

use std::fs::File;

use serenity::model::channel::Message;

#[path = "structs.rs"]
mod structs;
#[path = "user.rs"]
mod user;

use structs::{ApiTokens, ApiTokensFile};

// Search the tokens file to get the token of the item with the same id
pub fn get_api_token(id: &str) -> Result<Option<String>, serde_yaml::Error> {
  let f = File::open("wkdata/api_tokens.yaml").unwrap();
  let api_tokens: ApiTokensFile = serde_yaml::from_reader(f)?;
  let user_token = match api_tokens.tokens {
    Some(api_tokens) => api_tokens.into_iter().find(|r| r.id == id),
    None => None,
  };

  match user_token {
    Some(user_token) => Ok(Some(user_token.token.to_owned())),
    None => Ok(None),
  }
}

pub fn add_api_token(msg: &Message) -> Result<String, serde_yaml::Error> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();

  // Match for exactly 2 args (if anything else invalid)
  let response = match content_chunks.len() {
    2 => {
      let api_token = content_chunks[1];

      // Ensure apitoken is valid
      let user = user::get_user(&api_token);

      if user.is_err() {
        return Ok(format!("The Api Token given was invalid"));
      };

      // Read old file
      let api_token_f = File::open("wkdata/api_tokens.yaml").unwrap();
      let api_tokens: ApiTokensFile = serde_yaml::from_reader(&api_token_f)?;

      // New user tokens vec
      let mut new_user_tokens: Vec<ApiTokens> = match api_tokens.tokens {
        Some(tokens) => tokens,
        None => vec![],
      };

      // Find and remove the current value if value already exists
      // TODO: Remove user
      let removed_token = new_user_tokens
        .iter()
        .position(|token| *token.token == api_token.to_owned())
        .map(|e| new_user_tokens.remove(e))
        .is_some();
      println!("Removed existing Token: {}", removed_token);

      // Add to end of users (safe to unwrap user since we checked if err above)
      new_user_tokens.push(ApiTokens {
        id: user.unwrap().data.id,
        token: api_token.to_owned(),
      });

      // Write new yaml file
      let new_api_tokens = ApiTokensFile {
        tokens: Some(new_user_tokens),
      };
      let buffer = File::create("wkdata/api_tokens.yaml").expect("Failed to create file");
      serde_yaml::to_writer(buffer, &new_api_tokens)?;
      // Send confirmation
      format!("Successfully added new token: {}", api_token.to_owned())
    }
    _ => "The input is invalid, Example: !add_wkapi <api_token>".to_owned(),
  };
  Ok(response)
}
