use crate::parser::{
    buffer::Buffer,
    lexer::{
        scanners::{array::ARRAY_END, common::COMMA, object::OBJECT_END},
        tokens::{number_token::NumberToken, whitespace_token::is_whitespace},
    },
};
use std::{char, pin::Pin};

pub fn is_first_char_of_number(c: char) -> bool {
    match c {
        '-' => true,
        '+' => true,
        '0'..'9' => true,
        _ => false,
    }
}

enum NumberParseTerminationReason {
    /// Many reasons: I/O error, EOF, bad input, etc...
    Fatal(&'static str),
    /// This is because a ',', '}', or ']' got parsed so the number is natually over
    EndOfNumber,
}

pub async fn scan_token(buffer: &mut Pin<Box<&mut Buffer>>) -> Result<NumberToken, &'static str> {
    /*
     * This is the value returned for an integer number unless it has an exponent, or a decimal.
     * In that case it will use this as the first half of the number
     */
    let initial_number: i64 = 0;
    let mut return_val: NumberToken = NumberToken::Integer(0);

    loop {
        let res = buffer.next_char().await;

        if res.is_err() {
            return Err(res.err().unwrap());
        }

        let c = res.unwrap();
        let parse_state: Option<NumberParseTerminationReason> = match c {
            COMMA | OBJECT_END | ARRAY_END => {
                buffer.replace_char(c).await;
                Some(NumberParseTerminationReason::EndOfNumber)
            }
            c if is_whitespace(c) => None,
            '0'..'9' => {
                // TODO: handle digits
                None
            }
            'e' | 'E' => {
                // TODO: handle exponent
                None
            }
            '-' => {
                // TODO: handle negative number prefix/power prefix
                None
            }
            '+' => {
                // TODO: handle positive power prefix
                None
            }
            _ => Some(NumberParseTerminationReason::Fatal(
                "Invalid next character",
            )),
        };

        if parse_state.is_some() {
            return match parse_state {
                Some(NumberParseTerminationReason::Fatal(x)) => Err(x),
                Some(NumberParseTerminationReason::EndOfNumber) => Ok(return_val),
                _ => Err("An unknown error occurred"),
            };
        }
    }

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
