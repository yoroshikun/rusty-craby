use twilight_command_parser::Arguments;

fn describe_command(word: &str) -> Result<String, String> {
  match word {
      "jisho" => {
        Ok("Search for a specified word or phrase \n\n**Usage** !jisho <word> \n**Example** !jisho hello \n**Alias**: j, J".to_owned())
      }
      "xe" => {
        Ok("Check current exchange rates \n\n**Usage** !xe <from> <to> <amount> \n**Example** !xe AUD JPY 800 \n**Alias**: xe \n**Set Default** !exdefault AUD \n**Shorthand (uses default)** !xe or !xe <from> \n to currency is always JPY".to_owned())
      }
      "tubby" => {
        Ok("Organize Genshin Tubby requests \n\n**Usage** !tubby <create | complete>? <user>?\n**Alias**: t".to_owned())
      }
      "waifu" => {
        Ok("Generate a waifu with AI \n\n**Usage** !waifu \n**Alias**: w, uwu".to_owned())
      }
    //   "levels" => {
    //     Ok("List current levels of registered wkapi users \n\n**Usage** !levels \n\n**Alpha**".to_owned())
    //   }
    //   "add_wkapi" => {
    //     Ok("Add a new api key to the system \n\n**Usage** !add_api <api_key> \n\n**Important** Use api key v2 \n\n**Alpha**".to_owned())
    //   }
      _ => Err("No command found with that name".to_owned()),
    }
}

pub fn handler(mut arguments: Arguments) -> Result<String, String> {
  let count = arguments.clone().count();
  match count {
      1 => {
        let command = arguments.next().unwrap();
        describe_command(command)
      }
      0 => {
        Ok("List of available commands \n**jisho**: Search Jisho for a word or phrase \n**xe**: Check current exchange rates \n **tubby**: Organize Genshin Tubby requests \n**waifu**: Generate a waifu with AI".to_owned())
      }
      _ => Err("The input is invalid, Example: !help jisho".to_owned()),
    }
}
