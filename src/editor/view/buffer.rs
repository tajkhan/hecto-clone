
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self { lines: vec!["Hello, wolrd!".to_string()] }
    }

}

impl Buffer {

    pub fn is_empty(&self) -> bool {
        self.lines.len()==0
    }

}
