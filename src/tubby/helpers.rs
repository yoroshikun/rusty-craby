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

pub fn get_requests() -> Result<Vec<String>, String> {
    let config = read_config().expect("Could not read tubby file");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    let response = match config.users {
        Some(users) => {
            if users.is_empty() {
                Err("No requests available, use '!tubby create' to request one!".to_owned())
            } else {
                Ok(users
                    .iter()
                    .map(|user| {
                        format!(
                            "{} -> Remaining {:.2}h",
                            user.name.to_owned(),
                            (user.expires - current_time) / 60 / 60 // Fix this to round properly
                        )
                    })
                    .collect())
            }
        }
        None => Err("No requests available, use '!tubby create' to request one!".to_owned()),
    };

    response
}

pub fn create_request(current_user: User) -> Result<String, String> {
    let file = read_config().expect("Could not read tubby file");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    match file.users {
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
                        expires: current_time + (60 * 60 * 12),
                    };

                    users.push(new_user);

                    let new_file = UserTubbyFile {
                        last_updated: current_time,
                        users: Some(users),
                    };

                    let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");
                    serde_json::to_writer(buffer, &new_file).expect("Failed to write config");
                    return Ok("Successfuly added request!, check with !tubby".to_owned());
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

            let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");
            serde_json::to_writer(buffer, &new_file).expect("Failed to write config");
            return Ok("Successfuly added request!, check with !tubby".to_owned());
        }
    }
}

pub fn complete_request(completed_user: &str) -> Result<String, String> {
    let file = read_config().expect("Could not read tubby file");

    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_err) => 0,
    };

    match file.users {
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

                    let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");

                    serde_json::to_writer(buffer, &new_file).expect("Failed to write config");

                    return Ok(
                        format!("Successfuly completed {}s request", completed_user).to_owned()
                    );
                }
                None => {
                    return Err("The user does not exist to complete".to_owned());
                }
            };
        }
        None => {
            return Err("No users exist to complete".to_owned());
        }
    }
}

// TODO: Support pruning
// fn prune_requests(current: UserTubbyFile) -> Result<UserTubbyFile, ()> {
//     let pruned_users = current.users.filter(
//         user => user.timeout >= new Date().getTime()
//       );
//       if (prunedUsers.length !== current.users.length) {
//         const newConfig = {...current, users: prunedUsers} as TubbyFile;
//         await saveConfig(newConfig);
//         return newConfig;
//       }
//       return current;
// }

// Experimental expire
// pub fn expire_requests() -> Result<(), ()> {
//     let file = read_config().expect("Could not read tubby file");

//     let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
//         Ok(n) => n.as_secs(),
//         Err(_err) => 0,
//     };

//     match file.users {
//         Some(mut users) => {
//             let expired_users = users
//                 .iter()
//                 .clone()
//                 .filter(|user| &user.expires <= &current_time)
//                 .collect::<Vec<&UserTubbyUser>>();

//             if expired_users.is_empty() {
//                 return Ok(());
//             }

//             for expired_user in &expired_users {
//                 let index = users.iter().position(|user| {
//                     &user.name.to_ascii_lowercase() == &expired_user.name.to_ascii_lowercase()
//                 });

//                 if index.is_some() {
//                     new_users.remove(index.unwrap());
//                 }
//             }

//             let new_file = UserTubbyFile {
//                 last_updated: current_time,
//                 users: Some(users),
//             };

//             let buffer = fs::File::create(CONFIG_PATH).expect("Failed to create file");

//             serde_json::to_writer(buffer, &new_file).expect("Failed to write config");

//             return Ok(());
//         }
//         None => {
//             return Ok(());
//         }
//     }
// }
