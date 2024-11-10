use std::collections::HashMap;
use crate::helpers::world_manager::Entity;

pub struct Directory<'a> {
  
  pub name: String,
  pub files: HashMap<String, File>,
  pub children: &mut'a HashMap<String, Directory<'a>>,
  pub parent: Option<&'a Directory<'a>>,
  
} impl<'a> Directory<'_> {
  
  pub fn new(name: String, parent: Option<&Directory>) -> Directory<'a> {
    
    Directory {
      name,
      files: HashMap::new(),
      children: HashMap::new(),
      parent,
    }
  }

  pub fn add_child(&self, name: String) {
    
    self.children.insert(name, Directory::new(name, &self))
  }

  pub fn add_file(&mut self, file: File) {
    
    self.files.insert(file.name, file);
  }
} impl Entity for Directory {
  
  pub fn tick(self) -> Directory {
    
    for file in self.files {file.tick():
  }
}

pub struct File {
  
  pub name: String,
  pub content: Vec<String>,
  pub runstate: i32,

} impl Entity for Directory {
  
  fn tick(self) -> File {
    
    if self.runstate != -1 {
      
      //code interpretation
      self.runstate+=1;
    }
    
    self
  }
}