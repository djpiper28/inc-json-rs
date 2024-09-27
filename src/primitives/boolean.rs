use std::string::String;

const BOOLEAN_TRUE: &str = "true";
const BOOLEAN_FALSE: &str = "false";

pub fn is_first_char_of_boolean(c: char) -> bool {
    match c {
        't' => true,
        'f' => true,
        _ => false,
    }
}
