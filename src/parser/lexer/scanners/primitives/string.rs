use crate::parser::{buffer::Buffer, lexer::tokens::string_token::StringToken};
use std::char;

/// The maximum string length is a Gigabyte so that really long valid strings will terminate.
const MAX_READ_LENGTH: usize = 1024 * 1024 * 1024;

pub fn is_first_char_of_string(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}

pub struct StringParsingState {
    token: StringToken,
}

enum ScannedCharType {
    NormalCharacter,
    EscapedCharacter,
    StringEnd,
}

enum CharScanResult {
    Ok,
    EndOfToken,
    Err(&'static str),
}

fn char_type(c: char) -> ScannedCharType {
    return match c {
        '\\' => ScannedCharType::EscapedCharacter,
        _ => ScannedCharType::NormalCharacter,
    };
}

impl StringParsingState {
    pub fn new() -> Self {
        return StringParsingState {
            token: StringToken::new(),
        };
    }

    fn parse_unicode_escape_sequence(&mut self, buffer: &mut Buffer) -> Result<(), &'static str> {
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

    fn parse_escape_sequence(&mut self, buffer: &mut Buffer) -> Result<(), &'static str> {
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

    fn scan_char(&mut self, c: char, buffer: &mut Buffer) -> CharScanResult {
        match char_type(c) {
            ScannedCharType::NormalCharacter => {
                self.token.add_char(c);
                return CharScanResult::Ok;
            }
            ScannedCharType::EscapedCharacter => match self.parse_escape_sequence(buffer) {
                Err(x) => {
                    return CharScanResult::Err(x);
                }
                Ok(()) => CharScanResult::Ok,
            },
            ScannedCharType::StringEnd => CharScanResult::EndOfToken,
        }
    }

    pub fn scan_token(&mut self, buffer: &mut Buffer) -> Result<StringToken, &'static str> {
        for _ in 0..MAX_READ_LENGTH {
            let scan_result = match buffer.next_char() {
                Err(x) => Err(x),
                Ok(c) => Ok(self.scan_char(c, buffer)),
            };

            let return_val: Option<Result<StringToken, &'static str>>;
            match scan_result {
                Ok(CharScanResult::Err(x)) | Err(x) => {
                    return_val = Some(Err(x));
                }
                Ok(CharScanResult::EndOfToken) => {
                    return_val = Some(Ok(self.token.clone()));
                }
                Ok(CharScanResult::Ok) => return_val = None,
            };

            if return_val.is_some() {
                return return_val.unwrap();
            }
        }

        return Err("Exceeded maximum length");
    }
}
