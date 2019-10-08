use std::fs;
use std::io::Error;
use std::path::Path;

use serde_yaml;

#[path = "structs.rs"]
mod structs;

use structs::{ApiTokensFile, UserLevelsFile};

fn ensure_base_dir() -> Result<(), Error> {
  if !Path::new("wkdata").exists() {
    fs::create_dir("wkdata")?;
  };

  Ok(())
}

fn ensure_api_tokens_file() -> Result<(), Error> {
  if !Path::new("wkdata/api_tokens.yaml").is_file() {
    let f = fs::File::create("wkdata/api_tokens.yaml")?;
    let default = ApiTokensFile { tokens: None };
    serde_yaml::to_writer(f, &default).expect("Failed to write default yaml");
  };

  Ok(())
}

fn ensure_levels_file() -> Result<(), Error> {
  if !Path::new("wkdata/levels.yaml").is_file() {
    let f = fs::File::create("wkdata/levels.yaml")?;
    let default = UserLevelsFile {
      last_updated: 0,
      users: None,
    };
    serde_yaml::to_writer(f, &default).expect("Failed to write default yaml");
  };

  Ok(())
}

// Ensure all
pub fn ensure_all() -> Result<(), Error> {
  ensure_base_dir()?;
  ensure_api_tokens_file()?;
  ensure_levels_file()?;

  Ok(())
}
