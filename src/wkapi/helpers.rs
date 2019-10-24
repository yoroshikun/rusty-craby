use std::fs;

use serde_yaml;

#[path = "structs.rs"]
mod structs;

use structs::{ApiTokensFile, UserLevels, UserLevelsFile};

pub fn read_api_tokens_file() -> Result<ApiTokensFile, serde_yaml::Error> {
  let f = fs::File::open("wkdata/api_tokens.yaml").expect("Could not open file");
  let api_tokens_file: ApiTokensFile = serde_yaml::from_reader(f)?;
  Ok(api_tokens_file)
}

pub fn read_levels_file() -> Result<UserLevelsFile, serde_yaml::Error> {
  let f = fs::File::open("wkdata/levels.yaml").expect("Could not open file");
  let user_levels_file: UserLevelsFile = serde_yaml::from_reader(f)?;
  Ok(user_levels_file)
}

// Ensure (amount of ids in api_tokens == lower level file)
pub fn ensure_equal_ids_levels() -> Result<(), std::io::Error> {
  // Read files
  let api_tokens = read_api_tokens_file().unwrap().tokens;
  let users = read_levels_file().unwrap().users;

  // If length of api tokens is 0 return true (which is check for null)
  match api_tokens {
    Some(tokens) => {
      match users {
        Some(users) => {
          // If both tokens and users are the same size dont update
          if users.len() == tokens.len() {
            return Ok(());
          }

          // Reset levels with all ids (from api_tokens file)
          let mut new_levels = vec![];
          for id in tokens {
            new_levels.push(UserLevels {
              username: "".to_owned(),
              id: id.id,
              level: 0,
            });
          }
          let new_levels_f = UserLevelsFile {
            last_updated: 0,
            users: Some(new_levels),
          };
          // Write new file
          let buffer = fs::File::create("wkdata/levels.yaml")?;
          serde_yaml::to_writer(buffer, &new_levels_f).expect("Failed to write yaml");
          return Ok(());
        }
        None => {
          // Reset levels with all ids (from api_tokens file)
          let mut new_levels = vec![];
          for id in tokens {
            new_levels.push(UserLevels {
              username: "".to_owned(),
              id: id.id,
              level: 0,
            });
          }
          let new_levels_f = UserLevelsFile {
            last_updated: 0,
            users: Some(new_levels),
          };
          // Write new file
          let buffer = fs::File::create("wkdata/levels.yaml")?;
          serde_yaml::to_writer(buffer, &new_levels_f).expect("Failed to write yaml");
          return Ok(());
        }
      }
    }
    None => {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "No tokens in tokens file",
      ))
    }
  }
}
