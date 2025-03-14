

use crossterm::event::{KeyEventKind, KeyEventState};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

mod terminal;
use terminal::Terminal;
pub struct Editor {

    should_quit: bool,

}

impl Editor{

    pub const fn new() -> Self {
        Editor{
            should_quit: false,
        }
    }
    
    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), std::io::Error>{
        loop { 
            
        
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            _ = self.evaluate_event(&event);
 
        }
        Ok(())
    }

    fn evaluate_event(&mut self , event : &Event)-> Result<(), std::io::Error>{
        if let Key(KeyEvent{kind:_, modifiers, code, state:_}) = event {
          match code {
              Char('q') if *modifiers == KeyModifiers::CONTROL => {
                self.should_quit = true;
                
              }
              _ => (),
              
          }
        }
        Ok(())
        
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            _ = Terminal::clear_screen();
            print!("Goodbye.\r\n");
        }else{
            _ = Self::draw_rows();
            _ = Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
        
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let  height = Terminal::size().unwrap().1;
        for this_row in 0..height  {
            print!("~");
            if this_row < height - 1 {
                print!("\r\n");
            }
        }
   
        Ok(())
    }
}