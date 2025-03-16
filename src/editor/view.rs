


mod buffer;
use buffer::Buffer;
use super::terminal::{Position, Size, Terminal} ;
use std::io::Error;

const NAME: &str = "Echo";
const VERSION: &str = "0.1.0";

pub struct View{
    buffer: Buffer,
    size: Size,
    needs_redraw: bool,
}
impl View{
    

    pub fn render_line(line: &str , row : usize) -> Result<(), Error> {

        Terminal::move_cursor_to(Position { x: 0, y: row })?;

        Terminal::clear_line()?;
        Terminal::print(line)?;
        Ok(())
    }

    

    

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw {
            return Ok(());
        }
        
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }
        
        for this_row in 0..height {
            if let Some(line) = self.buffer.lines.get(this_row) {
                let truncated_line = if line.len() > width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(truncated_line, this_row)?; // Use render_line instead of direct prints
            } else if this_row == height / 2 && self.buffer.lines.is_empty() {
                let mut welcome_message: String = format!("{NAME} editor -- version {VERSION}");
                let len: usize = welcome_message.len();
                if width <= len {
                    Self::render_line("~", this_row)?;
                } else {
                    let padding = (width - len) / 2;
                    let spaces = " ".repeat(padding - 1);
                    welcome_message = format!("~{spaces}{welcome_message}");
                    welcome_message.truncate(width);
                    Self::render_line(&welcome_message, this_row)?; // Use render_line here too
                }
            } else {
                Self::render_line("~", this_row)?;
            }
        }
        
        self.needs_redraw = false;
        Ok(())
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

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(), 
        }
    }
}