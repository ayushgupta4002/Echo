use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
// use std::fmt::Display;
use std::io::{stdout, Error, Write}; 

#[derive(Debug , Default, Clone, Copy)]
pub struct Size{
    pub width: usize,
    pub height: usize
}


#[derive(Debug , Clone, Copy, Default)]
pub struct Position{
    pub x: usize,
    pub y: usize
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self { 
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
 pub struct Terminal {}
 
 impl Terminal {
     pub fn terminate() -> Result<(), std::io::Error> {
        Self::leave_alternate_screen()?;
        // Self::show_cursor()?;
        Self::execute()?;

         disable_raw_mode()?;
         Ok(())
     }
     pub fn initialize() -> Result<(), std::io::Error> {
         enable_raw_mode()?;
         Self::enter_alternate_screen()?;
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

    pub fn enter_alternate_screen() -> Result<(), Error> {
        queue!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), Error> {
        queue!(stdout(), LeaveAlternateScreen)?;

        Ok(())
    }
     pub fn move_cursor_to(position : Position) -> Result<Position, std::io::Error> {
         queue!(stdout(), MoveTo(position.x as u16, position.y as u16))?;

         
         Ok(Position { x: position.x, y: position.y })
     }
     pub fn size() -> Result<Size, Error> { 
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

    pub fn print_row(line: &str , row : usize) -> Result<(), Error> {
        Self::move_cursor_to(Position { x: 0, y: row })?;
        Self::clear_line()?;
        Self::print(line)?;
        Ok(())
    }

 }