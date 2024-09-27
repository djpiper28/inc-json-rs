const NULL: &str = "null";

pub fn is_first_char_of_null(c: char) -> bool {
    match (c) {
        'n' => true,
        _ => false,
    }
}
