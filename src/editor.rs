use crossterm::event::{read, Event, Event::Key, KeyCode, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;

mod term;
use term::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    cursor_pos: Position,
}


impl Editor {

    pub fn default() -> Self {
        Self {should_quit: false, cursor_pos: Position{x:0, y:0}}
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();   // called from mod term
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {

            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {

        let Size{height, width} = Terminal::size().unwrap(); // returns incorrect height!!
        let height= 55 as usize;    // fixing height

        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Left => {if self.cursor_pos.x>0 {self.cursor_pos.x -=1;}},
                KeyCode::Right => {if self.cursor_pos.x<width {self.cursor_pos.x +=1;}},
                KeyCode::Up => {if self.cursor_pos.y>0 {self.cursor_pos.y -=1;}},
                KeyCode::Down => {if self.cursor_pos.y<height {self.cursor_pos.y +=1;}},
                KeyCode::Home => {self.cursor_pos.x=0;},
                KeyCode::End => {self.cursor_pos.x=width;},
                KeyCode::PageUp => {self.cursor_pos.y=0;},
                KeyCode::PageDown => {self.cursor_pos.y=height;},

                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.cursor_pos)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width + len) / 2;

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

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?; // returns incorrect height!!
        let height = 55 as usize;    // fixing height

        Terminal::move_cursor_to(Position{x:0, y:0})?;

        for current_row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height/3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            } 

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        println!("=== {0:?}", Terminal::size());
        Ok(())
    }
}
