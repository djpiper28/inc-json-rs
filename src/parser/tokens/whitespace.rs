pub fn is_whitespace(c: char) -> bool {
    match c {
        '\n' => true,
        '\r' => true,
        '\t' => true,
        ' ' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test_whitespace_primitive {
    use super::*;

    #[test]
    fn test_is_whitespace_not_whitespace_is_false() {
        assert_eq!(is_whitespace('0'), false);
        assert_eq!(is_whitespace('n'), false);
        assert_eq!(is_whitespace('t'), false);
        assert_eq!(is_whitespace('a'), false);
        assert_eq!(is_whitespace('{'), false);
        assert_eq!(is_whitespace('['), false);
        assert_eq!(is_whitespace('"'), false);
        assert_eq!(is_whitespace(':'), false);
        assert_eq!(is_whitespace(','), false);
    }
}
