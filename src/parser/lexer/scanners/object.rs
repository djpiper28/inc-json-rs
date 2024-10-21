const OBJECT_VALUE_INDICATOR: char = ':';
const OBJECT_START: char = '{';
const OBJECT_END: char = '}';

pub fn is_object_start(c: char) -> bool {
    return c == OBJECT_START;
}

pub fn is_object_end(c: char) -> bool {
    return c == OBJECT_END;
}

pub fn is_object_value_indicator(c: char) -> bool {
    return c == OBJECT_VALUE_INDICATOR;
}
