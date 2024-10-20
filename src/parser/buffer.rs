use super::json_path::JsonPath;

pub type BufferChunk = Vec<char>;

/// Stores a buffer of incoming characters as a vector of strings (in `Vec<char>` form).
pub struct Buffer {
    buffers: Vec<BufferChunk>,
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
