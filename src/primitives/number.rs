pub fn is_first_char_of_number(c: char) -> bool {
    match (c) {
        '-' => true,
        '+' => true,
        '0'..'9' => true,
        _ => false,
    }
}
