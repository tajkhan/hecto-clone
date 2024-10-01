use super::term::{Position, Terminal, Size};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod buffer;
use buffer::Buffer;

pub struct View {
    buf: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {

    pub fn load(&mut self, filename: &str) {
        if let Ok(buffer) = Buffer::load(filename) {
            self.buf = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position{row: at, col:0})?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    pub fn render(&mut self) -> Result<(), Error> { // mut making because needs_redraw=false
        if !self.needs_redraw {
            return Ok(());
        }

        let Size{height, width} = self.size;
        if height==0 || width==0 {
            return Ok(());
        }
        let height = 20 as usize;    // fixing height

        #[allow(clippy::integer_division)]
        let vertical_center = height/3;

        for current_row in 0..height {
            if let Some(line) = self.buf.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            } else if current_row == vertical_center && self.buf.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}
