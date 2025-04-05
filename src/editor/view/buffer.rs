use std::{fs::{read_to_string, File}, io::{Error, Write}};

use super::{line::{self, Line}, Location};

#[derive(Default,Debug)]
pub struct Buffer {
    pub lines: Vec<Line>, 
    pub file_name : Option<String>,
    pub dirty: bool, 
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
        Ok(Self { lines, file_name : Some(filename.to_string()) , dirty : false})
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = File::create(file_name)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.lines.len() { 
            return;
        }
        if at.line_index == self.lines.len() { 
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index); 
            self.dirty = true;
        }
        
    }

    pub fn delete(&mut self, at: Location) {
        if at.line_index >= self.lines.len() {
            return;
        }
        let line_count = self.lines.len();
        if let Some(line) = self.lines.get_mut(at.line_index) {

            if at.grapheme_index >= line.grapheme_count() && line_count > at.line_index.saturating_add(1){
             

                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;


            }
            else if at.grapheme_index< line.grapheme_count(){
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;

            }

        }
    
    }

    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index >= self.lines.len() {
            self.lines.push(Line::default());
            self.dirty = true;

        }
        else if let Some(line) = self.lines.get_mut(at.line_index){
            let new_line = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new_line);
    
            }
        }
}