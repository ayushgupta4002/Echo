use std::{fs::read_to_string, io::Error};

use super::{line::Line, Location};

#[derive(Default,Debug)]
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

    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.lines.len() { 
            return;
        }
        if at.line_index == self.lines.len() { 
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index); 
        }
    }
}