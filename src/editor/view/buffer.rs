use std::{fs::read_to_string, io::Error};


#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String> 
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn load_file(filename : &str)-> Result<Self, Error>{
        let contents = read_to_string(filename).unwrap();
        let mut lines = Vec::new();
        for line in contents.lines(){
            lines.push(line.to_string());
        }
        Ok(Self { lines })
    }
}