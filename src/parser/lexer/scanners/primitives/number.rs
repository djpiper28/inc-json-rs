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

struct NumberParsingState {
    parts: [i64; 3],
    current_part: usize,
    number_negative: bool,
    exponent_negative: bool,
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
                    self.exponent_negative = true;
                }
                None
            }
            '.' => {
                if self.current_part == 0 {
                    self.current_part += 1;
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
                    Some(NumberParseTerminationReason::EndOfNumber) => {
                        Ok(NumberToken::Integer(123))
                    }
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
