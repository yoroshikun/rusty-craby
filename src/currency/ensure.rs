use std::fs;
use std::io::Error;
use std::path::Path;

use serde_yaml;

#[path = "structs.rs"]
mod structs;

use structs::PersonalizationCurrencyFile;

fn ensure_personalization_dir() -> Result<(), Error> {
  if !Path::new("personalization").exists() {
    fs::create_dir("personalization")?;
  };

  Ok(())
}

fn ensure_currency_personalization_file() -> Result<(), Error> {
  if !Path::new("personalization/currency.yaml").is_file() {
    let f = fs::File::create("personalization/currency.yaml")?;
    let default = PersonalizationCurrencyFile {
      last_updated: 0,
      users: None,
    };
    serde_yaml::to_writer(f, &default).expect("Failed to write default yaml");
  };

  Ok(())
}

// Ensure all
pub fn ensure_all() -> Result<(), Error> {
  ensure_personalization_dir()?;
  ensure_currency_personalization_file()?;

  Ok(())
}
