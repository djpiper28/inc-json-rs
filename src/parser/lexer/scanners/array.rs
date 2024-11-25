const ARRAY_START: char = '[';
pub const ARRAY_END: char = ']';

pub fn is_first_char_of_array_start(c: char) -> bool {
    return c == ARRAY_START;
}

pub fn is_first_char_of_array_end(c: char) -> bool {
    return c == ARRAY_END;
}
