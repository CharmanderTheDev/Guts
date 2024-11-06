use crate::helpers::text_loader::TextLoader;


pub struct CommandParser<'a> {

    pub textloader: TextLoader<'a>,

}impl CommandParser<'_> {

    pub fn new<'a>() -> CommandParser<'a> {
        CommandParser {
            textloader: TextLoader::new()
        }
    }

    pub fn parse_command(&self, command: &str) -> Result<String, &str>{

        match &command[0..command.len() - 2] {

            

            "HELP" => {
                self.textloader.load_text("HELP")
            }

            "HQUIT" => {
                println!("Goodbye Agent");
                std::process::exit(0)
            }

            _=>{Ok("Command not recognized. Try HELP".to_string())}
        }
    }
}
