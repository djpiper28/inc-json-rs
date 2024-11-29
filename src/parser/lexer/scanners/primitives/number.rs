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

const NOT_SET: i64 = -1;
const TEN: f64 = 10.0;

struct NumberParsingState {
    parts: [i64; 3],
    current_part: usize,
    number_negative: bool,
    exponent_negative: bool,
}

fn as_sign_multiplier(is_negative: bool) -> i64 {
    if is_negative {
        return -1;
    } else {
        return 1;
    }
}

impl NumberParsingState {
    fn new() -> Self {
        return NumberParsingState {
            parts: [NOT_SET, NOT_SET, NOT_SET],
            current_part: 0,
            number_negative: false,
            exponent_negative: false,
        };
    }

    fn as_number_token(&self) -> NumberToken {
        let base = self.parts[0] * as_sign_multiplier(self.number_negative);

        let decimal_part_as_int = self.parts[1];
        let mut decimal_part = decimal_part_as_int as f64;

        if self.current_part > 0 && decimal_part_as_int == NOT_SET {
            panic!("Corrupt state of number parser - part[1] is not set")
        }

        if self.current_part > 0 {
            while decimal_part > 1.0 {
                decimal_part /= TEN;
            }
            decimal_part *= as_sign_multiplier(self.number_negative) as f64;
        }

        match self.current_part {
            0 => {
                return NumberToken::Integer(base);
            }
            1 => {
                return NumberToken::Float(base as f64 + decimal_part);
            }
            2 => {
                let exponent_multiplier = TEN.powf(
                    (self.parts[2] as f64) * (as_sign_multiplier(self.exponent_negative) as f64),
                );
                return NumberToken::Float(base as f64 + decimal_part * exponent_multiplier);
            }
            _ => panic!("Corrupt state of number parser - current_part is > 2"),
        }
    }

    /**
     * Scans the next char of a number, if the char is not part of a number then it returns
     * it to the buffer.
     */
    async fn scan_char(
        self: &mut Self,
        c: char,
        buffer: &mut Pin<Box<&mut Buffer>>,
    ) -> Option<NumberParseTerminationReason> {
        return match c {
            COMMA | OBJECT_END | ARRAY_END => {
                buffer.replace_char(c).await;
                Some(NumberParseTerminationReason::EndOfNumber)
            }
            c if is_whitespace(c) => None,
            '0'..'9' => {
                if self.parts[self.current_part] == NOT_SET {
                    self.parts[self.current_part] = 0;
                }

                self.parts[self.current_part] *= 10;
                // This is known safe
                self.parts[self.current_part] += c.to_digit(10).unwrap() as i64;
                None
            }
            'e' | 'E' => {
                if self.current_part != 1 {
                    return Some(NumberParseTerminationReason::Fatal(
                        "Expected the exponent to follow digits from the decimal",
                    ));
                }

                if self.parts[self.current_part] == NOT_SET {
                    return Some(NumberParseTerminationReason::Fatal(
                        "Expected the post-decimal part of the number to not be empty",
                    ));
                }

                self.current_part += 1;
                None
            }
            '-' => {
                if self.current_part == 0 {
                    self.number_negative = true;
                } else if self.current_part == 1 {
                    return Some(NumberParseTerminationReason::Fatal(
                        "The post-decimal part of the number cannot have a sign",
                    ));
                } else {
                    self.exponent_negative = true;
                }
                None
            }
            '+' => {
                if self.current_part == 0 {
                    return Some(NumberParseTerminationReason::Fatal("A postive sign at the start of a number is not supported in the ECMA script"));
                } else if self.current_part == 1 {
                    return Some(NumberParseTerminationReason::Fatal(
                        "The post-decimal part of the number cannot have a sign",
                    ));
                } else {
                    self.exponent_negative = false;
                }
                None
            }
            '.' => {
                if self.current_part == 0 && self.parts[0] != NOT_SET {
                    self.current_part = 1;
                    return None;
                } else {
                    return Some(NumberParseTerminationReason::Fatal(
                        "A decimal can only appear in a number once, before the exponent",
                    ));
                }
            }
            _ => Some(NumberParseTerminationReason::Fatal(
                "Invalid next character",
            )),
        };
    }

    async fn scan_token(
        self: &mut Self,
        first_char: char,
        buffer: &mut Pin<Box<&mut Buffer>>,
    ) -> Result<NumberToken, &'static str> {
        let parse_state = self.scan_char(first_char, buffer).await;

        if parse_state.is_some() {
            return match parse_state {
                Some(NumberParseTerminationReason::Fatal(x)) => Err(x),
                Some(NumberParseTerminationReason::EndOfNumber) => {
                    Err("The first character of a number should not be the end of the number")
                }
                _ => Err("An unknown error occurred"),
            };
        }

        loop {
            let res = buffer.next_char().await;

            if res.is_err() {
                return Err(res.err().unwrap());
            }

            let c = res.unwrap();
            let parse_state = self.scan_char(c, buffer).await;

            if parse_state.is_some() {
                return match parse_state {
                    Some(NumberParseTerminationReason::Fatal(x)) => Err(x),
                    Some(NumberParseTerminationReason::EndOfNumber) => Ok(self.as_number_token()),
                    _ => Err("An unknown error occurred"),
                };
            }
        }
    }
}

enum NumberParseTerminationReason {
    /// Many reasons: I/O error, EOF, bad input, etc...
    Fatal(&'static str),
    /// This is because a ',', '}', or ']' got parsed so the number is natually over
    EndOfNumber,
}

pub async fn scan_number_token(
    first_char: char,
    buffer: &mut Pin<Box<&mut Buffer>>,
) -> Result<NumberToken, &'static str> {
    return NumberParsingState::new()
        .scan_token(first_char, buffer)
        .await;
}

#[cfg(test)]
mod test_number_primitive {
    use super::*;
    use std::borrow::BorrowMut;

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

    #[tokio::test]
    async fn test_scan_number_token_base_case_with_first_char_read() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "123,"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());

        assert!(is_first_char_of_number(
            buffer_pinned.next_char().await.unwrap()
        ));

        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(123));
    }

    #[tokio::test]
    async fn test_scan_number_token_base_case_ends_with_comma() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "23,"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(123));
    }

    #[tokio::test]
    async fn test_scan_number_token_base_case_ends_with_object_end() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "23}"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(123));
    }

    #[tokio::test]
    async fn test_scan_number_token_base_case_ends_with_array_end() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "23]"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(123));
    }

    #[tokio::test]
    async fn test_scan_number_token_base_case_ends_with_whitespace() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "23         ]"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(123));
    }

    #[tokio::test]
    async fn test_scan_number_token_negative_int() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "123,"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('-', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Integer(-123));
    }

    #[tokio::test]
    async fn test_scan_number_token_float() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                ".23,"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Float(1.23));
    }

    #[tokio::test]
    async fn test_scan_number_token_negative_float() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "5.12,"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('-', buffer_pinned).await;
        assert_eq!(ret.unwrap(), NumberToken::Float(-5.12));
    }

    #[tokio::test]
    async fn test_scan_number_token_invalid_first_char() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "bcd"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('a', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_token_invalid_plus_sign_first_char() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "123"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('+', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_token_negative_after_decimal() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                ".-23"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_token_plus_after_decimal() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                ".+23"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_token_invalid_second_char() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "bcd"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('1', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_first_char_is_decimal() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "222"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('.', buffer_pinned).await;
        assert!(ret.is_err());
    }

    #[tokio::test]
    async fn test_scan_number_first_char_is_exponent() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "222"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        let ret = scan_number_token('e', buffer_pinned).await;
        assert!(ret.is_err());
    }
}
