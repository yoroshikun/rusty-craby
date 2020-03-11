use serde::{Deserialize, Serialize};

// API Tokens file
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiTokensFile {
  pub tokens: Option<Vec<ApiTokens>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiTokens {
  pub id: String,
  pub token: String,
}

// Levels file
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLevels {
  pub username: String,
  pub id: String,
  pub level: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLevelsFile {
  pub last_updated: u64,
  pub users: Option<Vec<UserLevels>>,
}

// User API Call
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
  pub profile_url: String,
  pub started_at: String,
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
