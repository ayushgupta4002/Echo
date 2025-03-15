use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::fmt::Display;
use std::io::{stdout, Error, Write}; 

#[derive(Debug , Clone, Copy)]
pub struct Size{
    pub width: usize,
    pub height: usize
}


#[derive(Debug , Clone, Copy, Default)]
pub struct Position{
    pub x: usize,
    pub y: usize
}
 pub struct Terminal {}
 
 impl Terminal {
     pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
         disable_raw_mode()?;
         Ok(())
     }
     pub fn initialize() -> Result<(), std::io::Error> {
         enable_raw_mode()?;
         Self::clear_screen()?;
         Self::execute()?;
         Ok(())
     }
     pub fn clear_screen() -> Result<(), std::io::Error> {
         queue!(stdout(), Clear(ClearType::All))?;
         Ok(())
     }
     pub fn print (string: &str)  -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
     pub fn move_cursor_to(position : Position) -> Result<(), std::io::Error> {
         queue!(stdout(), MoveTo(position.x as u16, position.y as u16))?;
         Ok(())
     }
     pub fn size() -> Result<Size, std::io::Error> { 
        let (width, height) = size()?;
         Ok(Size{width: width.into(), height: height.into()})
     }
     pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(()) 
    }   
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?; 
        Ok(())
    }

 }