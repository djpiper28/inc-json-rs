use std::char;

use crate::parser::{buffer::Buffer, lexer::tokens::string_token::StringToken};

pub fn is_first_char_of_string(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}

pub struct StringParsingState {
    token: StringToken,
}

enum ParsingCharResult {
    NormalCharacter,
    EscapedCharacter,
}

fn char_type(c: char) -> ParsingCharResult {
    return match c {
        '\\' => ParsingCharResult::EscapedCharacter,
        _ => ParsingCharResult::NormalCharacter,
    };
}

impl StringParsingState {
    pub fn new() -> Self {
        return StringParsingState {
            token: StringToken::new(),
        };
    }

    fn parse_unicode_escape_sequence(mut self, mut buffer: Buffer) -> Result<(), &'static str> {
        let mut c: u32 = 0;

        for i in 0..4 {
            // index 0 -> 3
            //       1 -> 2
            //       2 -> 1
            //       3 -> 0
            let offset = 4 - i - 1;

            let res: Result<(), &'static str> = match buffer.next_char() {
                Err(x) => Err(x),
                Ok(next_char) => match u32::from_str_radix(next_char.to_string().as_str(), 16) {
                    Err(_) => Err("Invalid hex digit in unicode escape sequence"),
                    Ok(char_value) => {
                        c |= char_value << offset;
                        Ok(())
                    }
                },
            };

            if res.is_err() {
                return res;
            }
        }

        match char::from_u32(c) {
            Some(c) => {
                self.token.add_char(c);
                Ok(())
            }
            None => Err("Invalid character in escape sequence"),
        }
    }

    fn parse_escape_sequence(mut self, mut buffer: Buffer) -> Result<(), &'static str> {
        match buffer.next_char() {
            Err(x) => Err(x),
            Ok(first_char) => {
                match first_char {
                    'u' => {
                        return self.parse_unicode_escape_sequence(buffer);
                    }
                    '"' => {
                        self.token.add_char('"');
                        Ok(())
                    }
                    '\\' => {
                        self.token.add_char('\\');
                        Ok(())
                    }
                    '/' => {
                        // TODO: check this
                        // https://english.stackexchange.com/questions/10993/what-is-the-difference-between-solidus-and-slash#10996
                        self.token.add_char('⁄');
                        Ok(())
                    }
                    'b' => {
                        // TODO: check this
                        self.token.add_char('\u{0008}');
                        Ok(())
                    }
                    'f' => {
                        // TODO: check this
                        self.token.add_char('\u{000C}');
                        Ok(())
                    }
                    'n' => {
                        self.token.add_char('\n');
                        Ok(())
                    }
                    'r' => {
                        self.token.add_char('\r');
                        Ok(())
                    }
                    't' => {
                        self.token.add_char('\t');
                        Ok(())
                    }
                    _ => Err("Not a valid escape sequence"),
                }
            }
        }
    }

    pub fn parse(mut self, mut buffer: Buffer) -> Result<(), &'static str> {
        match buffer.next_char() {
            Err(x) => Err(x),
            Ok(c) => match char_type(c) {
                ParsingCharResult::NormalCharacter => {
                    self.token.add_char(c);
                    return Ok(());
                }
                ParsingCharResult::EscapedCharacter => {
                    return self.parse_escape_sequence(buffer);
                }
            },
        }
    }
}
