use std::collections::HashMap;
use std::io::Error;
use std::path::Path;
use std::time::SystemTime;
use std::{env, fs};

use serde_json;

#[path = "structs.rs"]
mod structs;

use structs::{UserXEFile, UserXEUser};

const CONFIG_PATH: &str = "user/xe.json";

fn ensure_user_dir() -> Result<(), Error> {
    let parent = Path::new(CONFIG_PATH).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent)?;
    };

    Ok(())
}

fn ensure_xe_user_file() -> Result<(), Error> {
    if Path::new(CONFIG_PATH).is_file() == false {
        let f = fs::File::create(CONFIG_PATH)?;

        let default = UserXEFile {
            last_updated: 0,
            users: None,
        };

        serde_json::to_writer(f, &default).expect("Failed to write default config");
    };

    Ok(())
}

pub fn ensure_all_files() -> Result<(), Error> {
    ensure_user_dir()?;
    ensure_xe_user_file()?;

    Ok(())
}

pub fn set_default(default: &str, id: &u64) -> Result<String, String> {
    let file = read_config().expect("Could not read xe user file");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    match file.users {
        Some(mut users) => {
            let index = users.iter().position(|user| &user.id == id);

            match index {
                Some(index) => {
                    users.remove(index);
                }
                None => {}
            };

            let new_user = UserXEUser {
                id: *id,
                default: default.to_ascii_uppercase(),
            };

            users.push(new_user);

            let new_file = UserXEFile {
                last_updated: current_time,
                users: Some(users),
            };

            let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");

            serde_json::to_writer(buffer, &new_file).expect("Failed to write config");

            return Ok("Successfuly updated currency default".to_owned());
        }
        None => {
            let new_user = UserXEUser {
                id: *id,
                default: default.to_ascii_uppercase(),
            };

            let new_file = UserXEFile {
                last_updated: current_time,
                users: Some(vec![new_user]),
            };

            let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");

            serde_json::to_writer(buffer, &new_file).expect("Failed to write config");

            return Ok("Successfuly updated currency default".to_owned());
        }
    }
}

pub fn get_default(id: u64) -> Result<String, String> {
    let file = read_config().expect("Could not read xe user file");

    let response = match file.users {
        Some(users) => {
            let index = users.iter().position(|user| &user.id == &id);

            match index {
                Some(index) => Ok(users[index].default.to_owned()),
                None => Err(
                    "No user default set, use !xe default <default> to set a default".to_owned(),
                ),
            }
        }
        None => Err("No user defaults set, use !xe default <default> to set a default".to_owned()),
    };

    response
}

fn read_config() -> Result<UserXEFile, serde_json::Error> {
    let f = fs::File::open(CONFIG_PATH).expect("Could not open file");
    let config_file: UserXEFile = serde_json::from_reader(f)?;
    Ok(config_file)
}

pub async fn get_xe_rate<S: Into<String>>(
    from: S,
    to: S,
) -> Result<f64, Box<dyn std::error::Error>> {
    // Perform the conversion to String
    let from = from.into();
    let to = to.into();
    let conversion = format!("{}_{}", from, to);

    let api_key = env::var("CURR_CONV_TOKEN").expect("Expected XE Access Token");
    // Form and send request
    let request_url = format!(
        "https://free.currconv.com/api/v7/convert?q={}&compact=ultra&apiKey={}",
        &conversion, &api_key
    );
    let exchange: HashMap<String, f64> = reqwest::get(&request_url).await?.json().await?;

    // Print debug if in debug mode
    if cfg!(debug_assertions) {
        println!("{:?}", exchange.get(&conversion));
    }

    let rate = exchange.get(&conversion).unwrap();

    Ok(rate.to_owned())
}
