use std::collections::HashMap;
use crate::helpers::directory_manager::Directory;

pub struct Entity<'a> {

    pub subentities: HashMap<String, Entity>,

} impl<'a> Entity<'_> {
    
    pub new() -> Entity {
        
        Entity {
            HashMap::new(),
        }
    }

    pub tick(self)  -> Entity {
    
    }
}