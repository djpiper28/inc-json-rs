use super::tokens::JsonToken;
use crate::parser::buffer::Buffer;
use array::{is_first_char_of_array_end, is_first_char_of_array_start};
use common::is_first_char_of_comma;
use object::{
    is_first_char_of_object_end, is_first_char_of_object_start,
    is_first_char_of_object_value_indicator,
};
use primitives::{
    boolean::{is_first_char_of_boolean, scan_boolean_token},
    null::{is_first_char_of_null, scan_null_token},
    number::{is_first_char_of_number, scan_number_token},
    string::{is_first_char_of_string, scan_string_token},
};
use std::boxed::Box;
use std::pin::Pin;

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
        return match scan_null_token(buffer).await {
            Ok(_) => Ok(JsonToken::Null),
            Err(x) => Err(x),
        }
    } else if is_first_char_of_boolean(c) {
        return match scan_boolean_token(c, buffer).await {
            Ok(x) => Ok(JsonToken::Boolean(x)),
            Err(x) => Err(x),
        }
    } else if is_first_char_of_number(c) {
        return match scan_number_token(c, buffer).await {
            Ok(x) => Ok(JsonToken::Number(x)),
            Err(x) => Err(x),
        }
    } else if is_first_char_of_string(c) {
        return match scan_string_token(buffer).await {
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
