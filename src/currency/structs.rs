use serde::{Deserialize, Serialize};

// Personalization
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalizationCurrencyFile {
  pub last_updated: u64,
  pub users: Option<Vec<PersonalizationCurrencyUser>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalizationCurrencyUser {
  pub id: u64,
  pub default: String,
}
