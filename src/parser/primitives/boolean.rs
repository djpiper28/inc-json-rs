const BOOLEAN_TRUE: &str = "true";
const BOOLEAN_FALSE: &str = "false";

pub fn is_first_char_of_boolean(c: char) -> bool {
    match c {
        't' | 'f' => true,
        _ => false,
    }
}
