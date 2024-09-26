use core::cmp::min;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyModifiers,
};

use std::io::Error;

mod term;
use term::{Terminal, Size, Position};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}


impl Editor {

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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        
        let Size { height, width } = Terminal::size()?; // returns incorrect height
        let height= 55 as usize;    // fixing height

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


    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {

        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Left |
                KeyCode::Right |
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Home |
                KeyCode::End |
                KeyCode::PageUp |
                KeyCode::PageDown => {
                    self.move_point(*code)?;
                },

                _ => (),
            }
        }

        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
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

        Terminal::move_caret_to(Position{col:0, row:0})?;

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
