use super::json_path::JsonPath;
use std::string::String;

struct Buffer {
    buffer: String,
    current_path: JsonPath,
    /// Whether or not there is more data to be expected after the end of the buffer.
    more_data: bool,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            buffer: String::new(),
            current_path: Vec::new(),
            more_data: true,
        }
    }
}
