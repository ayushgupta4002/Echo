

use crossterm::event::{read, Event::{self, Key}, KeyCode::{self}, KeyEvent, KeyEventKind, KeyModifiers};
use std::panic::{set_hook, take_hook};
use std::{env, io::Error};
mod terminal;
use terminal::{Terminal, Position , Size };
mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view :View
}


impl Editor{
    

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
                        self.view.move_point(code);
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
        _ = Terminal::move_cursor_to(self.view.get_position());
        
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