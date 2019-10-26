#[path = "structs.rs"]
mod structs;

use std::fs;
use std::time::SystemTime;

use structs::{PersonalizationCurrencyFile, PersonalizationCurrencyUser};

pub fn set_default(default: &str, id: &u64) -> Result<String, String> {
  let file =
    read_currency_personalization_file().expect("Could not read currency personalization file");
  let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => n.as_secs(),
    Err(_err) => 0,
  };
  // Find and delete user
  match file.users {
    Some(mut users) => {
      let index = users.iter().position(|user| &user.id == id);

      match index {
        Some(index) => {
          users.remove(index);
        }
        None => {}
      };

      let new_user = PersonalizationCurrencyUser {
        id: *id,
        default: default.to_ascii_uppercase(),
      };
      users.push(new_user);

      let new_file = PersonalizationCurrencyFile {
        last_updated: current_time,
        users: Some(users),
      };

      // Write new file
      let buffer =
        fs::File::create("personalization/currency.yaml").expect("Failed to create file");
      serde_yaml::to_writer(buffer, &new_file).expect("Failed to write yaml");
      return Ok("Successfuly updated currency default".to_owned());
    }
    None => {
      let new_user = PersonalizationCurrencyUser {
        id: *id,
        default: default.to_ascii_uppercase(),
      };

      let new_file = PersonalizationCurrencyFile {
        last_updated: current_time,
        users: Some(vec![new_user]),
      };

      // Write new file
      let buffer =
        fs::File::create("personalization/currency.yaml").expect("Failed to create file");
      serde_yaml::to_writer(buffer, &new_file).expect("Failed to write yaml");
      return Ok("Successfuly updated currency default".to_owned());
    }
  }
}

pub fn get_default(id: &u64) -> Result<String, String> {
  let file =
    read_currency_personalization_file().expect("Could not read currency personalization file");
  let response = match file.users {
    Some(users) => {
      let index = users.iter().position(|user| &user.id == id);

      match index {
        Some(index) => Ok(users[index].default.to_owned()),
        None => Err("No user default set, use !xe default <default> to set a default".to_owned()),
      }
    }
    None => Err("No user defaults set, use !xe default <default> to set a default".to_owned()),
  };

  response
}

pub fn read_currency_personalization_file() -> Result<PersonalizationCurrencyFile, serde_yaml::Error>
{
  let f = fs::File::open("personalization/currency.yaml").expect("Could not open file");
  let user_levels_file: PersonalizationCurrencyFile = serde_yaml::from_reader(f)?;
  Ok(user_levels_file)
}
