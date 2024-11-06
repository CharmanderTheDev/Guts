use std::io::{self, Write};
mod helpers;use helpers::{command_parse::CommandParser, directory_manager::Directory};

fn main() {
    
    let parser = CommandParser::new(Directory::new("Prometheus".to_string(), None));

    loop {

        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("std::io failed to read input");

        let parse = parser.parse_command(&input).unwrap();
        println!("{parse}");
    }
}