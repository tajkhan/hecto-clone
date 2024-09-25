use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crate::term::*;


pub struct Editor {
    should_quit: bool,
}


impl Editor {

    pub fn default() -> Self {
        Editor {should_quit: false}
    }

    pub fn run(&mut self) {
        initialize().unwrap();
        draw_rows().unwrap();
        let result = self.repl();
        terminate().unwrap();   // called from mod term
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {

            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },

                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            clear_screen()?;
            print!("Goodbye. \r\n");
        }

        Ok(())
    }

}
