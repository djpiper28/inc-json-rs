pub fn is_first_char_of_number(c: char) -> bool {
    match (c) {
        '-' => true,
        '+' => true,
        '0'..'9' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test_number_primitive {
    use super::*;

    #[test]
    fn test_is_first_char_of_number_digit() {
        for i in 0..9 {
            assert!(is_first_char_of_number(
                i.to_string().chars().next().unwrap()
            ));
        }
    }

    #[test]
    fn test_is_first_char_of_number_plus() {
        assert!(is_first_char_of_number('+'))
    }

    #[test]
    fn test_is_first_char_of_number_minus() {
        assert!(is_first_char_of_number('-'))
    }
}
