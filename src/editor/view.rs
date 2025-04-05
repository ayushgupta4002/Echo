


mod buffer;
use buffer::Buffer;
use crossterm::event::KeyCode;
use line::Line;
use super::{terminal::{ Position, Size, Terminal}, DocumentStatus} ;
use std::{cmp::min, io::Error};
mod line;
mod location;
const NAME: &str = "Echo";
const VERSION: &str = "0.1.0";

#[derive(Copy, Clone, Default, Debug)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

#[derive(Debug , Default)]
pub struct View{
    buffer: Buffer,
    size: Size,
    needs_redraw: bool,
    text_location: Location,
    scroll_offset: Position,
}
impl View{

    pub fn new(margin_bottom : usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Size { width: size.width, height: size.height.saturating_sub(margin_bottom) },
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }

    pub fn get_status(&self) -> DocumentStatus{
        DocumentStatus{
            total_lines: self.buffer.lines.len(),
            current_line_index: self.text_location.line_index,
            file_name: self.buffer.file_name.clone(),
            is_modified: self.buffer.dirty,
        }

    }

    pub fn render_line(line: &str , row : usize)  {

        let result = Terminal::print_row(line, row);
         debug_assert!(result.is_ok(),"Error rendering line: {:?}", result);
    }
    pub fn caret_position(&self) -> Position { 
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let y = self.text_location.line_index;
        let x = self.buffer.lines.get(y).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { x ,y } 
    }

    pub fn render(&mut self) {
        // println!("refresh_screen called, need_redraw: {}", self.needs_redraw);

        if !self.needs_redraw {
            return ;
        }

        
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return ;
        }
        let top = self.scroll_offset.y;
        for this_row in 0..height {
            if let Some(line) = self.buffer.lines.get(this_row.saturating_add(top)) {

                let left = self.scroll_offset.x;
                let right: usize= self.scroll_offset.x.saturating_add(width);
                // println!("Visible Graphemes: {:?}", line.get_visible_graphemes(left..right));


                Self::render_line(&line.get_visible_graphemes(left..right), this_row); 
            } else if this_row == height / 2 && self.buffer.lines.is_empty() {
                let mut welcome_message: String = format!("{NAME} editor -- version {VERSION}");
                let len: usize = welcome_message.len();
                if width <= len {
                    Self::render_line("~", this_row);
                } else {
                    let padding = (width - len) / 2;
                    let spaces = " ".repeat(padding - 1);
                    welcome_message = format!("~{spaces}{welcome_message}");
                    welcome_message.truncate(width);
                    Self::render_line(&welcome_message, this_row); // Use render_line here too
                }
            } else {
                Self::render_line("~", this_row);
            }
        }
        
        self.needs_redraw = false;
        return;
    }
    pub fn move_point(&mut self, key_code:KeyCode)  {
        let Location { mut grapheme_index, mut line_index } = self.text_location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => {
                line_index = line_index.saturating_sub(1);
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::Down => {
                line_index = line_index.saturating_add(1);
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::Left => {
                if grapheme_index>0 {
                    grapheme_index -=1;
                }
                else if  line_index>0 {
                    line_index -=1;
                    grapheme_index = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count); 
 
                }
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::Right => {
                let line_len = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count); 
                // print!("{:?}", self.buffer.lines.get(line_index));
                if grapheme_index < line_len{
                    grapheme_index +=1;

                }
                else{
                    line_index = line_index.saturating_add(1);
                    grapheme_index = 0;

                }
                self.set_data(grapheme_index, line_index);

                // grapheme_index =  grapheme_index.saturating_add(1);
            }
            KeyCode::PageUp => {
                line_index = 0;
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::PageDown => {
                line_index = height.saturating_sub(1);
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::Home => {
                grapheme_index = 0;
                self.set_data(grapheme_index, line_index);

            }
            KeyCode::End => {
                grapheme_index = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count);
                self.set_data(grapheme_index, line_index);
            }
            KeyCode::Char(character ) => {
                // print!("{:?}", character);
                self.insert_char(character);
            }
            KeyCode::Tab => {
                self.insert_char('\t');
            }
            KeyCode::Enter => {
                self.enter_key();
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Delete => {
                self.delete();

            }
            
            _ => (),
        }
    self.scroll_location_into_view();
   return;
}

    pub fn save(&mut self) {
        self.buffer.save().unwrap();
    }

    fn set_data(&mut self, mut grapheme_index: usize, mut line_index: usize) {
        grapheme_index = min(grapheme_index, self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count));
        line_index = min(line_index, self.buffer.lines.len());
        self.text_location = Location { grapheme_index, line_index };
    }

    fn enter_key(&mut self){
        self.buffer.insert_newline(self.text_location);
        let Location { mut grapheme_index, mut line_index } = self.text_location;

        let line_len = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count); 
        // print!("{:?}", self.buffer.lines.get(line_index));
        if grapheme_index < line_len{
            grapheme_index +=1;

        }
        else{
            line_index = line_index.saturating_add(1);
            grapheme_index = 0;

        }
        self.text_location = Location { grapheme_index, line_index };
        self.needs_redraw = true;

    }
    fn backspace(&mut self) {
        let Location { mut grapheme_index, mut line_index } = self.text_location;
        if grapheme_index>0 {
            grapheme_index -=1;
        }
        else if  line_index>0 {
            line_index -=1;
            grapheme_index = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count); 

        }
        if self.text_location.line_index !=0 || self.text_location.grapheme_index !=0 {
            self.text_location = Location { grapheme_index, line_index };

            self.delete();
        }
    }

    fn delete(&mut self){
        self.buffer.delete(self.text_location);
        self.scroll_location_into_view();

        self.needs_redraw = true;

    }
    

    fn insert_char(&mut self, character: char) {
        let Location { mut grapheme_index, mut line_index } = self.text_location;

        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count); 


        self.buffer.insert_char(character, self.text_location);

        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count); 
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            //move right for an added grapheme (should be the regular case)
            let line_len = self.buffer.lines.get(line_index).map_or(0, Line::grapheme_count); 
            if grapheme_index < line_len{
                grapheme_index +=1;

            }
            else{
                line_index = line_index.saturating_add(2);
                grapheme_index = 0;

            }
        }
        self.text_location = Location { grapheme_index, line_index };
        
        self.needs_redraw = true;

        // self.render();
    }

    pub fn resize(&mut self , size : Size) -> Result<(), Error> {
        self.size = size;
        self.scroll_location_into_view();
        self.needs_redraw = true;
        // Terminal::print(&format!("size: {:?}", size))?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        if let Ok(buffer) = Buffer::load_file(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
        Ok(())
    }
    pub fn scroll_location_into_view(&mut self) {
        let Location { grapheme_index, line_index } = self.text_location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;


        // Scroll vertically
        if line_index < self.scroll_offset.y {
            self.scroll_offset.y = line_index;
            offset_changed = true;
        } else if line_index >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = line_index.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if grapheme_index < self.scroll_offset.x {
            self.scroll_offset.x = grapheme_index;
            offset_changed = true;
        } else if grapheme_index >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = grapheme_index.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = self.needs_redraw || offset_changed;

    }

}

// impl Default for View {
//     fn default() -> Self {
//         Self {
//             buffer: Buffer::default(),
//             needs_redraw: true,
//             size: Terminal::size().unwrap_or_default(), 
//             text_location: Location::default(),
//             scroll_offset: Position::default(),
//         }
//     }
// }