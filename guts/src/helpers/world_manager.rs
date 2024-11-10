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
    
    pub processing_progress: f32,
    pub processing_speed: f32,
    
} impl<'a> Computer<'_> {

    pub fn new(max_memory: i32, max_processes: i32, processing_speed: f32) {
        
        Computer {
            directory: Directory::new(),
            
            current_memory: 0,
            max_memory,
            
            current_processes: 0,
            max_processes,
            
            processing_progress: 0.0,
            processing_speed,
        }
    }
        
} impl Entity for Computer {

    
}