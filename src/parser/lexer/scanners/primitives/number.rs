use crate::parser::{buffer::Buffer, lexer::tokens::number_token::NumberToken};
use std::{char, pin::Pin};

pub fn is_first_char_of_number(c: char) -> bool {
    match c {
        '-' => true,
        '+' => true,
        '0'..'9' => true,
        _ => false,
    }
}

pub async fn scan_token(buffer: &mut Pin<Box<&mut Buffer>>) -> Result<NumberToken, &'static str> {
    /*
     * This is the value returned for an integer number unless it has an exponent, or a decimal.
     * In that case it will use this as the first half of the number
     */
    let initial_number: i64 = 0;

    return Ok(NumberToken::Integer(7));
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
