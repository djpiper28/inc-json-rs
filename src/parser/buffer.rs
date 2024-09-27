use std::string::String;

struct Buffer {
    tmp_buffer: String,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            tmp_buffer: String::new(),
        }
    }
}
