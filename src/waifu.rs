use std::process::Command;

fn generate_waifu() -> Result<Vec<&'static str>, String> {
    let mut child = Command::new("auto-waifu")
        .spawn()
        .expect("auto-waifu command failed to start");
    let _result = child.wait().expect("Failed to wait for command to finish");
    Ok(vec!["avatar.png"])
}

pub fn handler() -> Result<String, String> {
    generate_waifu().expect("Failed to generate Waifu");
    Ok("Here is your waifu".to_owned())
}
