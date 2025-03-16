

use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
use std::{cell::RefMut, cmp::min, panic::{set_hook, take_hook}};
use std::{env, io::Error};
mod terminal;
use terminal::{Terminal, Position , Size };
mod view;
use view::View;
#[derive(Default, Debug ,Clone, Copy)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
    view :View

}


impl Editor{
    
    // pub fn run(&mut self){
    //     Terminal::initialize().unwrap();
    //     self.handle_args(); 
    //     let result = self.repl();
    //     Terminal::terminate().unwrap();
    //     result.unwrap();
    // }
    pub fn new()-> Result<Self, Error>{
        let current_hook = take_hook();

        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            let _ = view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })
    }
    // fn handle_args(&mut self) {
    //     let args: Vec<String> = env::args().collect();
    //     if let Some(file_name) = args.get(1) {
    //        _=  self.view.load(file_name);
    //     }
    // } 
    pub fn run(&mut self) {

        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }

    }

    fn move_point(&mut self, key_code:KeyCode)  {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
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
       return;
    }

    fn evaluate_event(&mut self , event : Event){
    
        match event {
            Key(KeyEvent{kind:KeyEventKind::Press, modifiers, code, state:_}) => {
                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
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
                        self.move_point(code);
                    }
                    _ => (),
                }
            }
            Event::Resize(width_u16,height_u16) => {
            let height_usize = height_u16 as usize;
            let width_usize = width_u16 as usize;
            _ = self.view.resize(Size{width: width_usize, height: height_usize});
            }
            
            _ => (),
            
        }
        return;
        
    }

    fn refresh_screen(&mut self) {
        Terminal::hide_cursor().unwrap();
        let _ = Terminal::move_cursor_to(Position::default());
      
        self.view.render();
        _ = Terminal::move_cursor_to(Position{x : self.location.x , y: self.location.y});
        
        let _ =Terminal::show_cursor();
        let _ = Terminal::execute();
        return;        
    }
    
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n"); 
        }
    }
}