use std::io::Error;
use std::fs::read_to_string;

use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {

    pub fn load(file_name: &str) -> Result<Self, Error> {

        let result = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in result.lines() {
             lines.push(Line::from(line));
        }
        Ok(Self{lines})
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len()==0
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }
}
