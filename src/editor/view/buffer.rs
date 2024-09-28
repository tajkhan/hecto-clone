use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {

    pub fn load(file_name: &str) -> Result<Self, Error> {

        let result = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for line in result.lines() {
             lines.push(line.to_string());
        }
        Ok(Self{lines})
    }

    pub fn is_empty(&self) -> bool {
        self.lines.len()==0
    }

}
