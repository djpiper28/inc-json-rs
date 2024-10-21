pub const NULL: &str = "null";

pub fn is_first_char_of_null(c: char) -> bool {
    match c {
        'n' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test_null_primitive {
    use super::*;

    #[test]
    fn test_null_is_first_char_of_null_is_true() {
        assert!(is_first_char_of_null(NULL.chars().next().unwrap()));
    }
}
