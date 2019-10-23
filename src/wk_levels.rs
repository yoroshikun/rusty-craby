use serde_yaml;

use serenity::model::channel::Message;

use reqwest;

use std::fs::File;
use std::time::SystemTime;

#[path = "wkapi/mod.rs"]
mod wkapi;

use wkapi::structs::{UserLevels, UserLevelsFile};

fn get_level_from_api(api_token: &str) -> Result<(String, String, u8), reqwest::Error> {
  let user = wkapi::user::get_user(&api_token).unwrap();
  Ok((user.data.username, user.data.id, user.data.level))
}

pub fn handler(_msg: &Message) -> String {
  // Ensure
  if wkapi::helpers::ensure_equal_ids_levels().is_err() {
    return "No API Tokens are available, please add one with !wkapi_add_token".to_owned();
  }

  // Read latest
  let levels = wkapi::helpers::read_levels_file().expect("Failed to read levels data");

  // Match for users
  let response = match levels.users {
    Some(users) => {
      // If the time since last update of this file is over an hour (3600 seconds) get all users levels again
      match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
          if levels.last_updated + 3600 <= n.as_secs() {
            // Init vec for holding new levels
            let mut new_vec: Vec<UserLevels> = vec![];
            // For each user on file we call the api
            for user in users {
              let api_token = wkapi::api_tokens::get_api_token(&user.id)
                .expect("Failed to get api token for user");

              // TODO Should I error here?
              let (username, id, level) = get_level_from_api(&api_token.unwrap()).unwrap_or((
                "".to_owned(),
                "".to_owned(),
                0,
              ));

              // Push new values to vector
              new_vec.push(UserLevels {
                username: username,
                id: id,
                level: level,
              })
            }

            // Write new yaml file
            let new_level_file = UserLevelsFile {
              last_updated: n.as_secs(),
              users: Some(new_vec),
            };
            let buffer = File::create("wkdata/levels.yaml").expect("Failed to create file");
            serde_yaml::to_writer(buffer, &new_level_file).expect("Failed to write yaml");

            // Loop though new users and return a string of joined username and levels
            let mut output_vec: Vec<String> = vec![];
            match new_level_file.users {
              Some(users) => {
                for user in users {
                  output_vec.push(format!("{}: {}", user.username, user.level))
                }
              }
              None => {}
            };
            format!("{}", output_vec.join(", "))
          } else {
            // Loop though users and return a string of joined username and levels
            let mut output_vec: Vec<String> = vec![];
            for user in users {
              output_vec.push(format!("{}: {}", user.username, user.level))
            }
            format!("{}", output_vec.join(", "))
          }
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
      }
    }
    None => "There are no users added to the levels database".to_owned(),
  };
  response
}
