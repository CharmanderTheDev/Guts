use std::{collections::HashMap, fs};

pub struct TextLoader<'a> {

    pub loaded_text: HashMap<&'a str, &'a str>,

}impl TextLoader<'_> {

    pub fn new<'a>() -> TextLoader<'a> {
        TextLoader {
            loaded_text: HashMap::new()
        }
    }

    pub fn load_text(&self, file: &str) -> Result<String, &str>{

        match self.loaded_text.get(file) {

            Some(text)=>Ok(text.to_string()),

            None=> {

                Ok(fs::read_to_string("./src/helpers/loadable_text/".to_owned()+file+".txt")
                .expect("TextLoader could not read the file"))
            }
        }
    }
}