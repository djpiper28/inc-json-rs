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

    pub fn add_data(mut self, data: BufferChunk) -> Result<Buffer, &'static str> {
        if self.eof {
            return Result::Err("Cannot add data once the EOF has occurred");
        }

        self.buffers.push(data);
        Ok(self)
    }

    pub fn eof(mut self) {
        self.eof = true;
    }
}

#[cfg(test)]
mod test_buffer {
    use super::*;
}
