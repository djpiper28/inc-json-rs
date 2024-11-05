use super::tokens::JsonToken;
use crate::parser::buffer::Buffer;
use array::{is_first_char_of_array_end, is_first_char_of_array_start};
use common::is_first_char_of_comma;
use object::{
    is_first_char_of_object_end, is_first_char_of_object_start,
    is_first_char_of_object_value_indicator,
};
use primitives::{
    boolean::is_first_char_of_boolean,
    null::is_first_char_of_null,
    number::is_first_char_of_number,
    string::{is_first_char_of_string, StringParsingState},
};
use std::boxed::Box;
use std::{borrow::Borrow, pin::Pin};

pub mod array;
pub mod common;
pub mod object;
pub mod primitives;

async fn scan_token(
    c: char,
    buffer: &mut Pin<Box<&mut Buffer>>,
) -> Result<JsonToken, &'static str> {
    if is_first_char_of_object_start(c) {
        return Ok(JsonToken::ObjectStart);
    } else if is_first_char_of_object_end(c) {
        return Ok(JsonToken::ObjectEnd);
    } else if is_first_char_of_object_value_indicator(c) {
        return Ok(JsonToken::ObejectValueIndicator);
    } else if is_first_char_of_array_start(c) {
        return Ok(JsonToken::ArrayStart);
    } else if is_first_char_of_array_end(c) {
        return Ok(JsonToken::ArrayEnd);
    } else if is_first_char_of_comma(c) {
        return Ok(JsonToken::Comma);
    } else if is_first_char_of_null(c) {
        todo!("Check that it is actually null");
    } else if is_first_char_of_boolean(c) {
        todo!("Check that it is actually a boolean, and what it is");
    } else if is_first_char_of_number(c) {
        todo!("Check that is is actually a number, and what it is");
    } else if is_first_char_of_string(c) {
        let mut string_scanner = StringParsingState::new();
        return match string_scanner.scan_token(buffer).await {
            Ok(x) => Ok(JsonToken::String(x)),
            Err(x) => Err(x),
        };
    } else {
        return Err("Cannot match a valid JSON token");
    }
}

pub async fn next_token(buffer: &mut Pin<Box<&mut Buffer>>) -> Result<JsonToken, &'static str> {
    match buffer.next_char().await {
        Ok(c) => {
            return scan_token(c, buffer).await;
        }
        Err(x) => Err(x),
    }
}
