use std::fs;
use std::io::Error;
use std::path::Path;
use std::time::SystemTime;
use twilight_model::user::User;

use serde_json;

#[path = "structs.rs"]
mod structs;

use structs::{UserTubbyFile, UserTubbyUser};

const CONFIG_PATH: &str = "user/tubby.json";

fn ensure_user_dir() -> Result<(), Error> {
    let parent = Path::new(CONFIG_PATH).parent().unwrap();
    if parent.exists() == false {
        fs::create_dir_all(parent)?;
    };

    Ok(())
}

fn ensure_tubby_user_file() -> Result<(), Error> {
    if Path::new(CONFIG_PATH).is_file() == false {
        let f = fs::File::create(CONFIG_PATH)?;

        let default = UserTubbyFile {
            last_updated: 0,
            users: None,
        };

        serde_json::to_writer(f, &default).expect("Failed to write default config");
    };

    Ok(())
}

pub fn ensure_all_files() -> Result<(), Error> {
    ensure_user_dir()?;
    ensure_tubby_user_file()?;

    Ok(())
}

fn read_config() -> Result<UserTubbyFile, serde_json::Error> {
    let f = fs::File::open(CONFIG_PATH).expect("Could not open file");
    let config_file: UserTubbyFile = serde_json::from_reader(f)?;
    Ok(config_file)
}

fn save_config(config: UserTubbyFile) -> Result<(), serde_json::Error> {
    let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create config file buffer");
    serde_json::to_writer(buffer, &config)?;
    Ok(())
}

fn make_user_list(users: Option<Vec<UserTubbyUser>>) -> String {
    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    match users {
        Some(users) => {
            if users.is_empty() {
                return "No requests available, use '!tubby request' to request one!".to_owned();
            } else {
                return users
                    .iter()
                    .map(|user| {
                        format!(
                            "{} -> Remaining {:.2}h \n",
                            user.name.to_owned(),
                            ((user.expires - current_time) as f64) / 60f64 / 60f64
                        )
                    })
                    .collect();
            }
        }
        None => "No requests available, use '!tubby request' to request one!".to_owned(),
    }
}

// TODO use above function to avoid redundancy
pub fn get_requests() -> Result<Vec<String>, String> {
    let file = read_config().expect("Could not read tubby file");
    let config = prune_requests(file).expect("Failed to prune requests");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };
    let response = match config.users {
        Some(users) => {
            if users.is_empty() {
                Err("No requests available, use '!tubby request' to request one!".to_owned())
            } else {
                Ok(users
                    .iter()
                    .map(|user| {
                        format!(
                            "{} -> Remaining {:.2}h",
                            user.name.to_owned(),
                            ((user.expires - current_time) as f64) / 60f64 / 60f64
                        )
                    })
                    .collect())
            }
        }
        None => Err("No requests available, use '!tubby request' to request one!".to_owned()),
    };

    response
}

pub fn create_request(current_user: User, offset: Option<u8>) -> Result<String, String> {
    let file = read_config().expect("Could not read tubby file");
    let config = prune_requests(file).expect("Failed to prune requests");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    let offset = match offset {
        Some(offset) => offset,
        None => 12,
    };

    match config.users {
        Some(mut users) => {
            let index = users.iter().position(|user| {
                &user.name.to_ascii_lowercase() == &current_user.name.to_ascii_lowercase()
            });

            match index {
                Some(_) => {
                    return Err("User request already lodged, check with !tubby".to_owned());
                }
                None => {
                    let new_user = UserTubbyUser {
                        name: current_user.name,
                        expires: current_time + (60 * 60 * offset as u64),
                    };

                    users.push(new_user);

                    let new_file = UserTubbyFile {
                        last_updated: current_time,
                        users: Some(users),
                    };

                    save_config(new_file.clone()).expect("Failed to save config");
                    return Ok(format!(
                        "Successfuly added request! \n **Requests** \n {}",
                        make_user_list(new_file.users)
                    ));
                }
            };
        }
        None => {
            let mut users = vec![];
            let new_user = UserTubbyUser {
                name: current_user.name,
                expires: current_time + (60 * 60 * 12),
            };

            users.push(new_user);

            let new_file = UserTubbyFile {
                last_updated: current_time,
                users: Some(users),
            };

            save_config(new_file.clone()).expect("Failed to save config");
            return Ok(format!(
                "Successfuly added request! \n **Requests** \n {}",
                make_user_list(new_file.users)
            ));
        }
    }
}

pub fn complete_request(completed_user: &str) -> Result<String, String> {
    let file = read_config().expect("Could not read tubby file");
    let config = prune_requests(file).expect("Failed to prune requests");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    match config.users.clone() {
        Some(mut users) => {
            let index = users.iter().position(|user| {
                &user.name.to_ascii_lowercase() == &completed_user.to_ascii_lowercase()
            });

            match index {
                Some(index) => {
                    users.remove(index);

                    let new_file = UserTubbyFile {
                        last_updated: current_time,
                        users: Some(users),
                    };

                    save_config(new_file.clone()).expect("Failed to save config");
                    return Ok(format!(
                        "Successfuly completed {}s request! \n **Requests** \n {}",
                        completed_user.to_ascii_lowercase(),
                        make_user_list(new_file.users)
                    ));
                }
                None => {
                    return Err(format!(
                        "The user does not exist to complete \n **Requests** \n {}",
                        make_user_list(config.users)
                    ))
                }
            };
        }
        None => {
            return Err("No users exist to complete".to_owned());
        }
    }
}

fn prune_requests(current: UserTubbyFile) -> Result<UserTubbyFile, ()> {
    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    let pruned_users = match current.users {
        Some(users) => users
            .into_iter()
            .filter(|user| user.expires >= current_time)
            .collect(),
        None => vec![],
    };

    let new_config = UserTubbyFile {
        last_updated: current_time,
        users: Some(pruned_users),
    };

    save_config(new_config.clone()).expect("Failed to save config");

    Ok(new_config)
}
