use crate::parser::{buffer::Buffer, lexer::tokens::string_token::StringToken};
use std::{char, pin::Pin};

/// The maximum string length is a Gigabyte so that really long valid strings will terminate.
const MAX_READ_LENGTH: usize = 1024 * 1024 * 1024;

pub fn is_first_char_of_string(c: char) -> bool {
    match c {
        '"' => true,
        _ => false,
    }
}

struct StringParsingState {
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
        '"' => ScannedCharType::StringEnd,
        _ => ScannedCharType::NormalCharacter,
    };
}

impl StringParsingState {
    pub fn new() -> Self {
        return StringParsingState {
            token: StringToken::new(),
        };
    }

    async fn parse_unicode_escape_sequence(
        &mut self,
        buffer: &mut Pin<Box<&mut Buffer>>,
    ) -> Result<(), &'static str> {
        let mut c: u32 = 0;

        for i in 0..4 {
            // index 0 -> 3 * 4
            //       1 -> 2 * 4
            //       2 -> 1 * 4
            //       3 -> 0 * 4
            let offset = (4 - i - 1) * 4;

            let res: Result<(), &'static str> = match buffer.next_char().await {
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

    async fn parse_escape_sequence(
        &mut self,
        buffer: &mut Pin<Box<&mut Buffer>>,
    ) -> Result<(), &'static str> {
        match buffer.next_char().await {
            Err(x) => Err(x),
            Ok(first_char) => match first_char {
                'u' => {
                    return self.parse_unicode_escape_sequence(buffer).await;
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
                    self.token.add_char('\u{002F}');
                    Ok(())
                }
                'b' => {
                    self.token.add_char('\u{0008}');
                    Ok(())
                }
                'f' => {
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
            },
        }
    }

    async fn scan_char(&mut self, c: char, buffer: &mut Pin<Box<&mut Buffer>>) -> CharScanResult {
        match char_type(c) {
            ScannedCharType::NormalCharacter => {
                self.token.add_char(c);
                return CharScanResult::Ok;
            }
            ScannedCharType::EscapedCharacter => match self.parse_escape_sequence(buffer).await {
                Err(x) => {
                    return CharScanResult::Err(x);
                }
                Ok(()) => CharScanResult::Ok,
            },
            ScannedCharType::StringEnd => CharScanResult::EndOfToken,
        }
    }

    async fn scan(
        &mut self,
        buffer: &mut Pin<Box<&mut Buffer>>,
    ) -> Result<StringToken, &'static str> {
        for _ in 0..MAX_READ_LENGTH {
            let scan_result = match buffer.next_char().await {
                Err(x) => Err(x),
                Ok(c) => Ok(self.scan_char(c, buffer).await),
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

/**
* This reads from the first char of the string
* For example `"abcdef..."`: `a` in the example is the first char
* Until the end of the string. The first char is expected to be read from
* `is_first_char_of_string()`
*/
pub async fn scan_string_token(
    buffer: &mut Pin<Box<&mut Buffer>>,
) -> Result<StringToken, &'static str> {
    return StringParsingState::new().scan(buffer).await;
}

#[cfg(test)]
mod test_string {
    use super::*;
    use std::borrow::BorrowMut;

    #[tokio::test]
    async fn test_string_scan_valid_base_case() {
        let mut buffer = Buffer::new();

        // This seems really convoluted to be hoenst
        assert!(buffer
            .add_data(
                "\"Hello world!\""
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());

        assert!(is_first_char_of_string(
            buffer_pinned.next_char().await.unwrap()
        ));

        let res = scan_string_token(buffer_pinned).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().as_string(), "Hello world!");
    }

    #[tokio::test]
    async fn test_string_scan_valid_escaped_quote() {
        let mut buffer = Buffer::new();

        // This seems really convoluted to be hoenst
        assert!(buffer
            .add_data(
                concat!(
                    '"',
                    "The man said:",
                    '\\',
                    '\"',
                    "you alright govna?",
                    '\\',
                    '"',
                    '"'
                )
                .to_string()
                .chars()
                .into_iter()
                .clone()
                .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());

        assert!(is_first_char_of_string(
            buffer_pinned.next_char().await.unwrap()
        ));

        let res = scan_string_token(buffer_pinned).await;

        assert!(res.is_ok());
        assert_eq!(
            res.unwrap().as_string(),
            concat!("The man said:", '\"', "you alright govna?", '"')
        );
    }

    #[tokio::test]
    async fn test_string_scan_valid_escaped_hex_code() {
        let mut buffer = Buffer::new();

        // This seems really convoluted to be hoenst
        assert!(buffer
            .add_data(
                "\"Hello \\u27bd do you like unicode?\""
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());

        assert!(is_first_char_of_string(
            buffer_pinned.next_char().await.unwrap()
        ));

        let res = scan_string_token(buffer_pinned).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().as_string(), "Hello ➽ do you like unicode?");
    }

    #[tokio::test]
    async fn test_string_scan_valid_escaped_backslash() {
        let mut buffer = Buffer::new();

        // This seems really convoluted to be hoenst
        assert!(buffer
            .add_data(
                "\">>\\\\<<\""
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());

        assert!(is_first_char_of_string(
            buffer_pinned.next_char().await.unwrap()
        ));

        let res = scan_string_token(buffer_pinned).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap().as_string(), ">>\\<<");
    }
}
