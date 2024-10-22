use super::json_path::JsonPath;

pub type BufferChunk = Vec<char>;

/// Stores a buffer of incoming characters as a vector of strings (in `Vec<char>` form).
pub struct Buffer {
    buffers: Vec<BufferChunk>,
    current_buffer_idx: usize,
    current_path: JsonPath,
    /// Whether or not there is more data to be expected after the end of the buffer.
    eof: bool,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            buffers: Vec::new(),
            current_path: Vec::new(),
            current_buffer_idx: 0,
            eof: false,
        }
    }

    pub fn add_data(&mut self, data: BufferChunk) -> Result<(), &'static str> {
        if self.eof {
            return Result::Err("Cannot add data once the EOF has occurred");
        }

        self.buffers.push(data);
        Result::Ok(())
    }

    /// Called when at the end of the buffer.
    pub fn eof(&mut self) {
        self.eof = true;
    }

    fn next_buffer(&mut self) {
        self.current_buffer_idx = 0;
        self.buffers.remove(0);
    }

    pub fn next_char(&mut self) -> Result<char, &'static str> {
        if self.eof {
            return Err("EOF reached");
        }

        return match self.buffers.first() {
            Some(buffer) => {
                if self.current_buffer_idx >= buffer.len() {
                    self.next_buffer();
                    return self.next_char();
                }

                let c = buffer[self.current_buffer_idx];
                self.current_buffer_idx += 1;
                return Ok(c);
            }
            None => Err("There are no more buffers to get characters from"),
        };
    }
}

#[cfg(test)]
mod test_buffer {
    use super::*;

    #[test]
    fn test_cannot_add_data_after_eof() {
        let mut buffer = Buffer::new();
        buffer.eof();

        let err = buffer.add_data(Vec::new());
        assert!(err.is_err(), "Should be in an error state");
    }
}
