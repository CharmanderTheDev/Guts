use std::collections::HashMap;
use crate::helpers::directory_managers::Directory;

pub trait Entity {
    
    pub fn tick(self) -> Entity;
}

pub struct Computer<'a> {

    pub directory: Directory,
    
    pub current_memory: i32,
    pub max_memory: i32,
    
    pub current_processes: i32,
    pub max_processes: i32,
    
    pub processing_speed: i32,
    
} impl<'a> Computer<'_> {

    pub fn new(max_processes: i32, 

} impl Entity for Computer {

    
}