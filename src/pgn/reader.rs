use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct PgnReader {
    position: usize,
    buffer: Vec<u8>,
}

impl PgnReader {
    pub fn new(path: &PathBuf) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .expect("Unable to read the file");

        Self {
            position: 0,
            buffer,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.buffer.len() == self.position {
            return None;
        }

        self.position += 1;
        Some(char::from(self.buffer[self.position - 1]))
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.buffer.len() == self.position {
            return None;
        }

        Some(char::from(self.buffer[self.position]))
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\r' || c == '\n' {
                self.next();
            } else {
                return;
            }
        }
    }

    pub fn read_to_whitespace(&mut self) -> Option<String> {
        if self.buffer.len() == self.position {
            return None;
        }

        let mut output = String::new();
        while let Some(c) = self.next() {
            if c == ' ' || c == '\r' || c == '\n' {
                self.skip_whitespace();
                return Some(output);
            }

            output.push(c);
        }

        None
    }

    pub fn read_to(&mut self, token: char) -> Option<String> {
        if self.buffer.len() == self.position {
            return None;
        }

        let mut output = String::new();
        while let Some(c) = self.next() {
            if c == token {
                return Some(output);
            }

            output.push(c);
        }

        None
    }
}
