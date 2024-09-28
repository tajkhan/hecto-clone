use super::term::{Terminal, Size};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buf: Buffer,
}

impl View {

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buf = buffer;
        }
    }

    pub fn render_welcome_screen()  -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?; // returns incorrect height!!
        let height = 55 as usize;    // fixing height

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
        //dummy debug
        println!("=== {0:?}", Terminal::size());
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?; // returns incorrect height!!
        let height = 55 as usize;    // fixing height

        for current_row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buf.lines.get(current_row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buf.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
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

}


