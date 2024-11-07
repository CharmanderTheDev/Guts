use std::collections::HashMap;

pub struct Directory<'a> {
  
  pub name: String,
  pub files: HashMap<String, File>,
  pub children: HashMap<String, Directory<'a>>,
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

  pub fn add_child(&mut self, name: String) {
    
    self.children.insert(name, Directory::new(name, &self))
  }

  pub fn add_file(&mut self, file: File) {
    
    self.files.insert(file.name, file);
  }
}

pub struct File {
  pub name: String,
  pub content: Vec<String>,
}