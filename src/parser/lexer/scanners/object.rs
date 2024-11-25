const OBJECT_VALUE_INDICATOR: char = ':';
const OBJECT_START: char = '{';
pub const OBJECT_END: char = '}';

pub fn is_first_char_of_object_start(c: char) -> bool {
    return c == OBJECT_START;
}

pub fn is_first_char_of_object_end(c: char) -> bool {
    return c == OBJECT_END;
}

pub fn is_first_char_of_object_value_indicator(c: char) -> bool {
    return c == OBJECT_VALUE_INDICATOR;
}
