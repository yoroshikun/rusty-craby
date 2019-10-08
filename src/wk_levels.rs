use serde::{Deserialize, Serialize};
use serde_yaml;

use serenity::model::channel::Message;

use reqwest;

use std::fs::File;
use std::path::Path;
use std::time::SystemTime;

#[path = "wkapi/mod.rs"]
mod wkapi;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UserLevels {
  username: String,
  id: String,
  level: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WKUserLevelMap {
  last_updated: u64,
  users: Vec<UserLevels>,
}

fn get_level_from_api(api_token: String) -> Result<u8, reqwest::Error> {
  let user = wkapi::user::get_user(api_token).unwrap();
  Ok(user.data.level)
}

fn read_levels_yaml() -> Result<WKUserLevelMap, serde_yaml::Error> {
  if Path::new("wkdata/levels.yaml").exists() {
    let wk_user_level_map: WKUserLevelMap = serde_yaml::from_reader(
      File::open("wkdata/levels.yaml").expect("Failed to open levels.yaml"),
    )?;
    Ok(wk_user_level_map)
  } else {
    if !Path::new("wkdata").exists() {
      std::fs::create_dir("wkdata");
    };

    let file = File::create("wkdata/levels.yaml").expect("Failed to create levels.yaml");
    let wk_user_level_map = WKUserLevelMap {
      last_updated: 0,
      users: vec![],
    };
    Ok(wk_user_level_map)
  }
}

pub fn handler(msg: &Message) -> String {
  // Get Latest YAML file
  let level_map = read_levels_yaml().expect("No Users have been added to the level database yet");
  // If users vec is none alert
  if level_map.users.is_empty() {
    "There are no users added to the levels database".to_owned()
  } else {
    // If the time since last update of this file is over an hour (3600 seconds) get all users levels again
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => {
        if level_map.last_updated + 3600 <= n.as_secs() {
          // Init vec for holding new levels
          let mut new_vec: Vec<UserLevels> = vec![];
          // For each user on file we call the api
          for user in level_map.users {
            let api_token = wkapi::api_tokens::get_api_token(user.id.to_owned())
              .expect("Failed to get api token for user");
            let new_level = get_level_from_api(api_token).unwrap_or(0);
            new_vec.push(UserLevels {
              username: user.username,
              id: user.id,
              level: new_level,
            })
          }

          // Write new yaml file
          let new_level_map = WKUserLevelMap {
            last_updated: n.as_secs(),
            users: new_vec,
          };
          let buffer = File::create("wkdata/levels.yaml").expect("Failed to create file");
          serde_yaml::to_writer(buffer, &new_level_map).expect("Failed to write yaml");

          // Loop though new users and return a string of joined username and levels
          let mut output_vec: Vec<String> = vec![];
          for user in new_level_map.users {
            output_vec.push(format!("{}: {}", user.username, user.level))
          }
          format!("{}", output_vec.join(", "))
        } else {
          // Loop though users and return a string of joined username and levels
          let mut output_vec: Vec<String> = vec![];
          for user in level_map.users {
            output_vec.push(format!("{}: {}", user.username, user.level))
          }
          format!("{}", output_vec.join(", "))
        }
      }
      Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
  }
}
