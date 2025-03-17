


mod buffer;
use buffer::Buffer;
use crossterm::event::KeyCode;
use line::Line;
use super::terminal::{ Position, Size, Terminal} ;
use std::{cmp::min, io::Error};
mod line;
mod location;
use location::Location;
const NAME: &str = "Echo";
const VERSION: &str = "0.1.0";
pub struct View{
    buffer: Buffer,
    size: Size,
    needs_redraw: bool,
    location: Location,
    scroll_offset: Location,
}
impl View{

    pub fn render_line(line: &str , row : usize)  {
        let result = Terminal::print_row(line, row);
         debug_assert!(result.is_ok(),"Error rendering line: {:?}", result);
    }
    pub fn get_position(&self) -> Position { 
        self.location.subtract(&self.scroll_offset).into()
    }
    pub fn render(&mut self) {
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


                Self::render_line(&line.get(left..right), this_row); 
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
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = y.saturating_add(1);
            }
            KeyCode::Left => {
                if x>0 {
                    x -=1;
                }
                else if  y>0 {
                    y -=1;
                    x = self.buffer.lines.get(y).map_or(0, Line::len); 
 
                }
            }
            KeyCode::Right => {
                let line_len = self.buffer.lines.get(y).map_or(0, Line::len); 
                if x < line_len{
                    x +=1;

                }
                else{
                    y = y.saturating_add(1);
                    x = 0;

                }

                x =  x.saturating_add(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = self.buffer.lines.get(y).map_or(0, Line::len);
            }
            _ => (),
        }

        x = min(x, self.buffer.lines.get(y).map_or(0, Line::len));
        y = min(y, self.buffer.lines.len());
        self.location = Location { x, y };
        
        
        self.scroll_location_into_view();
       return;
    }

    // fn draw_welcome_message() -> Result<(), Error> {
    //     let mut welcome_message: String = format!("{NAME} editor -- version {VERSION}");
    //     let width = Terminal::size()?.width as usize;
    //     let len = welcome_message.len();
    //     let padding = (width - len) / 2; 
    //     let spaces = " ".repeat(padding - 1);
    //     welcome_message = format!("~{spaces}{welcome_message}");
    //     welcome_message.truncate(width); 
    //     Terminal::print(&welcome_message)?;
    //     Ok(())
    // }
    // fn draw_empty_row() -> Result<(), Error> {
    //     Terminal::print("~")?;
    //     Ok(())
    // }

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
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;


        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(), 
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}