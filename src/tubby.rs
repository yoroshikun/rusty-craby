use twilight_command_parser::Arguments;
use twilight_model::user::User;

mod helpers;

pub fn handler(arguments: Arguments, user: User) -> Result<String, String> {
    let arguments_vec: Vec<&str> = arguments.collect();

    if arguments_vec.len() == 0 {
        return match helpers::get_requests() {
            Ok(requests) => Ok(requests.join("\n")),
            Err(err) => Err(err),
        };
    }

    match arguments_vec[0].to_ascii_lowercase().as_str() {
        "request" => {
            return helpers::create_request(user);
        }
        "create" => {
            return helpers::create_request(user);
        }
        "complete" => {
            let complete_user = arguments_vec[1];

            if complete_user.is_empty() {
                return Err("A user must be provided".to_owned());
            }
            return helpers::complete_request(complete_user);
        }
        "list " => {
            return match helpers::get_requests() {
                Ok(requests) => Ok(requests.join("\n")),
                Err(err) => Err(err),
            };
        }
        _ => {
            return match helpers::get_requests() {
                Ok(requests) => Ok(requests.join("\n")),
                Err(err) => Err(err),
            };
        }
    }
}

pub fn create_request(user: User) -> Result<String, String> {
    return helpers::create_request(user);
}

pub fn complete_request(user: &str) -> Result<String, String> {
    return helpers::complete_request(user);
}

pub fn get_requests() -> Result<String, String> {
    return match helpers::get_requests() {
        Ok(requests) => Ok(requests.join("\n")),
        Err(err) => Err(err),
    };
}

pub async fn ensure_all_files() -> Result<(), std::io::Error> {
    helpers::ensure_all_files()
}
