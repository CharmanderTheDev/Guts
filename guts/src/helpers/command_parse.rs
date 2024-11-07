use crate::helpers::{text_loader::TextLoader, directory_manager::Directory};

pub struct CommandParser<'a> {
    pub textloader: TextLoader<'a>,
    pub focus: Directory<'a>,
}
impl CommandParser<'_> {
    pub fn new<'a>(focus: Directory) -> CommandParser<'a> {
        CommandParser {
            textloader: TextLoader::new(),
            focus,
        }
    }

    pub fn parse_command(&self, command: &str) -> Result<String, &str> {
        
        //println!("{}", &command[0..command.len() - 2]);

        match &command[0..command.len() - 2] {
            "HELP" => self.textloader.load_text("HELP"),

            "HQUIT" => {
                println!("Goodbye Agent");
                std::process::exit(0)
            }

            _ => Ok("Command not recognized. Try HELP".to_string()),
        }
    }
}
