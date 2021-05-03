use twilight_command_parser::Arguments;
use twilight_model::user::User;

mod helpers;

pub fn handler(mut arguments: Arguments, user: User) -> Result<String, String> {
    let count = arguments.clone().count();

    match count {
        // Completing
        2 => {
            let command = arguments.next().unwrap();
            let user = arguments.next().unwrap();

            if command != "complete" {
                return Err("The input is invalid, Example !tubby complete <user>".to_owned());
            }

            return helpers::complete_request(user);
        }
        // Creating
        1 => {
            let command = arguments.next().unwrap();

            if command != "create" {
                return Err("The input is invalid, Example: !tubby create".to_owned());
            }

            return helpers::create_request(user);
        }
        // Listing
        0 => {
            // List
            return match helpers::get_requests() {
                Ok(requests) => Ok(requests.join("\n")),
                Err(err) => Err(err),
            };
        }
        _ => return Err("The input is invalid, Example: !tubby <command>".to_owned()),
    }
}

pub async fn ensure_all_files() -> Result<(), std::io::Error> {
    helpers::ensure_all_files()
}
