use serenity::model::channel::Message;

fn describe_command(word: &str) -> Result<String, String> {
  match word {
    "jisho" => {
      Ok("Search for a specified word or phrase \n\n**Usage** !jisho <word> \n**Example** !jisho hello \n**Alias**: j, J".to_owned())
    }
    "currency" => {
      Ok("Check current exchange rates \n\n**Usage** !currency <from> <to> <amount> \n**Example** !currency AUD JPY 800 \n**Alias**: xe \n**Set Default** !currency default AUD \n**Shorthand (uses default)** !xe or !xe <from> \n to currency is always JPY".to_owned())
    }
    "levels" => {
      Ok("List current levels of registered wkapi users \n\n**Usage** !levels \n\n**Alpha**".to_owned())
    }
    "add_wkapi" => {
      Ok("Add a new api key to the system \n\n**Usage** !add_api <api_key> \n\n**Important** Use api key v2 \n\n**Alpha**".to_owned())
    }
    _ => Err("No command found with that name".to_owned()),
  }
}

pub fn handler(msg: &Message) -> Result<String, String> {
  // Split command by spaces
  let content_chunks: Vec<&str> = msg.content.split(" ").collect();

  // Simple check to ensure word is given
  match content_chunks.len() {
    2 => {
      let command = content_chunks[1];
      describe_command(command)
    }
    1 => {
      Ok("List of available commands \n**jisho**: Search Jisho for a word or phrase \n**currency**: Check current exchange rates \n**levels**: List current levels of registered wkapi users \n**add_wkapi**: Add a new api key to the system".to_owned())
    }
    _ => Err("The input is invalid, Example: !jisho person".to_owned()),
  }
}
