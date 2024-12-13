use std::pin::Pin;

use super::{
    buffer::Buffer,
    json_path::{self, ObjectConsumer},
    lexer::scanners::scan_token,
};

pub struct Parser {
    json_path: ObjectConsumer,
}

impl Parser {
    pub fn new(json_path: ObjectConsumer) -> Self {
        return Self { json_path };
    }

    pub async fn parse(self, buffer: &mut Pin<Box<&mut Buffer>>) -> Result<(), &'static str> {
        loop {
            if buffer.is_eof().await {
                break;
            }

            let res = match buffer.next_char().await {
                Ok(c) => match scan_token(c, buffer).await {
                    Ok(_) => todo!("implement me"),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };

            if res.is_err() {
                return res;
            }
        }

        return Ok(());
    }
}
