


mod buffer;
use buffer::Buffer;
use super::terminal::Terminal ;
use std::io::Error;

const NAME: &str = "Echo";
const VERSION: &str = "0.1.0";

#[derive(Default)]
pub struct View{
    buffer: Buffer,
}
impl View{
    pub fn render_welcome_screen(&self)-> Result<(), Error>{
        let  size = Terminal::size().unwrap();
        for this_row in 0..size.height  {
            Terminal::clear_line()?;
            if this_row == size.height / 2 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if this_row < size.height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }
   
        Ok(())

    }

    fn render_buffer(&self) -> Result<(), Error> {
        let  size = Terminal::size().unwrap();
        for this_row in 0..size.height  {
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(this_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            }  else {
                Self::draw_empty_row()?;
                  if this_row < size.height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
            }
     
        }
   
        Ok(())
    }

    pub fn render (&self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            self.render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message: String = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2; 
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width); 
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        if let Ok(buffer) = Buffer::load_file(file_name) {
            self.buffer = buffer;
        }
        Ok(())
    }

}