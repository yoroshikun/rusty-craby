use serde::{Deserialize, Serialize};

use reqwest;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserBase {
  pub object: String,
  pub url: String,
  pub data_updated_at: String,
  pub data: UserData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserData {
  pub id: String,
  pub username: String,
  pub level: u8,
  pub max_level_granted_by_subscription: u8,
  pub profile_url: String,
  pub started_at: String,
  pub subscribed: bool,
  pub current_vacation_started_at: Option<String>,
  pub subscription: UserSubscription,
  pub preferences: UserPreferences,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSubscription {
  pub active: bool,
  pub r#type: String,
  pub max_level_granted: u8,
  pub period_ends_at: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPreferences {
  pub default_voice_actor_id: Option<u8>,
  pub lessons_autoplay_audio: bool,
  pub lessons_batch_size: u8,
  pub lessons_presentation_order: String,
  pub reviews_autoplay_audio: bool,
  pub reviews_display_srs_indicator: bool,
}

pub fn get_user(api_token: String) -> Result<UserBase, reqwest::Error> {
  let client = reqwest::Client::new();
  let mut res = client
    .get("https://api.wanikani.com/v2/user")
    .header("Authorization", format!("Bearer {}", api_token))
    .send()?;
  let user_base: UserBase = res.json()?;
  Ok(user_base)
}
