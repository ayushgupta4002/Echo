use std::{fs::read_to_string, io::Error};

use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line> 
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load_file(filename : &str)-> Result<Self, Error>{
        let contents = read_to_string(filename).unwrap();
        let mut lines = Vec::new();
        for line in contents.lines(){
            lines.push(Line::from(line));
        }
        Ok(Self { lines })
    }
}