use super::{terminal::{Size, Terminal}, DocumentStatus};



#[derive(Debug, Default)]
pub struct StatusBar{
    needs_redraw: bool,
    status: DocumentStatus,
    width: usize,
    margin_bottom: usize,
    position_y: usize,
}

impl StatusBar{

    pub fn new(margin_bottom : usize)-> Self{
        let size = Terminal::size().unwrap_or_default();
        Self{
            needs_redraw: true,
            status: DocumentStatus::default(),
            width: size.width,
            margin_bottom ,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.width = size.width;
        self.position_y = size.height.saturating_sub(self.margin_bottom).saturating_sub(1);
        self.needs_redraw = true;
    }

    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if(new_status == self.status){
            return;
        }
        self.status = new_status;
        self.needs_redraw = true;
    }

    pub fn render(&mut self){
        if self.needs_redraw != true {
            return;
        }
        let mut status = format!("{:?}", self.status); 
        status.truncate(self.width);
        let result = Terminal::print_row(&status, self.position_y);
        if result.is_ok() {
            self.needs_redraw = false;
        }
    }
}