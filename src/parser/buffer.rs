use super::json_path::JsonPath;
use std::string::String;

struct Buffer {
    buffer: String,
    current_path: JsonPath,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            buffer: String::new(),
            current_path: Vec::new(),
        }
    }
}
