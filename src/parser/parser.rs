use std::pin::Pin;

use super::{
    buffer::Buffer,
    json_path::ObjectConsumer,
    lexer::{scanners::scan_token, tokens::JsonToken},
};

enum CurrentlyScanning {
    Array,
    Object,
    /// The key is the value
    KeyValuePair(String),
}

pub struct Parser {
    json_path: ObjectConsumer,
}

impl Parser {
    pub fn new(json_path: ObjectConsumer) -> Self {
        return Self { json_path };
    }

    pub async fn parse(self, buffer: &mut Pin<Box<&mut Buffer>>) -> Result<(), &'static str> {
        let mut state_stack: Vec<CurrentlyScanning> = Vec::new();

        loop {
            if buffer.is_eof().await {
                break;
            }

            let res: Result<JsonToken, &'static str> = match buffer.next_char().await {
                Ok(c) => match scan_token(c, buffer).await {
                    Ok(token) => Ok(token),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };

            if res.is_err() {
                return Err(res.err().unwrap());
            }

            let parse_res: Result<(), &'static str> = match res.unwrap() {
                JsonToken::Whitespace => Ok(()),
                JsonToken::Null => todo!("handle me"),
                JsonToken::Boolean(boolean) => todo!("handle me"),
                JsonToken::Number(number) => todo!("handle me"),
                JsonToken::String(string) => todo!("handle me"),
                JsonToken::ObjectValueIndicator => todo!("handle me"),
                JsonToken::Comma => todo!("handle me"),
                JsonToken::ArrayStart => todo!("handle me"),
                JsonToken::ObjectStart => todo!("handle me"),
                JsonToken::ArrayEnd => todo!("handle me"),
                JsonToken::ObjectEnd => todo!("handle me"),
            };
        }

        return Ok(());
    }
}
