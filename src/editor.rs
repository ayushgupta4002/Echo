

use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
use std::{cmp::min, io::Error};

mod terminal;
use terminal::{Terminal, Position , Size };

#[derive(Default, Debug ,Clone, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,

}

const NAME: &str = "Echo";
const VERSION: &str = "0.1.0";

impl Editor{
    
    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), Error>{
        loop { 
            
        
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            _ = self.evaluate_event(&event)?;
 
        }
        Ok(())
    }

    fn move_point(&mut self, key_code:KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
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
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self , event : &Event)-> Result<(), Error>{
        if let Key(KeyEvent{kind:KeyEventKind::Press, modifiers, code, state:_}) = event {
          match code {
              KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
                
              }
              KeyCode::Up
              | KeyCode::Down
              | KeyCode::Left
              | KeyCode::Right
              | KeyCode::PageDown
              | KeyCode::PageUp
              | KeyCode::End
              | KeyCode::Home => {
                  self.move_point(*code)?;
              }
              _ => (),
              
          }
        }
        Ok(())
        
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            _ = Terminal::clear_screen();
            let _ = Terminal::print("Goodbye.\r\n");
        }else{
            _ = Self::draw_rows();
            _ = Terminal::move_cursor_to(Position{x : self.location.x , y: self.location.y})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
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
        Terminal::print(welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let  size = Terminal::size().unwrap();
        for this_row in 0..size.height  {
            Terminal::clear_line()?;
            if this_row == size.height / 2 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if this_row < size.height - 1 {
                Terminal::print("\r\n")?;
            }
        }
   
        Ok(())
    }



}