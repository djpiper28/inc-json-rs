use number_token::NumberToken;
use string_token::StringToken;

pub mod number_token;
pub mod string_token;
pub mod whitespace_token;

#[derive(PartialEq, Debug)]
pub enum JsonToken {
    Whitespace,
    Null,
    Boolean(bool),
    Number(NumberToken),
    String(StringToken),
    ObjectStart,
    ObjectEnd,
    /// The colon (:) that makes up the "key": "value" of an object entry
    ObjectValueIndicator,
    ArrayStart,
    ArrayEnd,
    Comma,
}
