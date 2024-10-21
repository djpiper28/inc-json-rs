const ARRAY_START: char = '[';
const ARRAY_END: char = ']';

pub fn is_array_start(c: char) -> bool {
    return c == ARRAY_START;
}

pub fn is_array_end(c: char) -> bool {
    return c == ARRAY_END;
}
