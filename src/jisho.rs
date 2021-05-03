use twilight_command_parser::Arguments;

mod helpers;

pub async fn handler(mut arguments: Arguments<'_>) -> Result<(String, String), String> {
    let count = arguments.clone().count();
    // Simple check to ensure word is given
    match count {
        1 => {
            let word = arguments.next().unwrap();
            helpers::format_jisho(word).await
        }
        _ => return Err("The input is invalid, Example: !jisho person".to_owned()),
    }
}
