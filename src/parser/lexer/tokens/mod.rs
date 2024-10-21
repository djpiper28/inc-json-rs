use number_token::NumberToken;
use string_token::StringToken;

pub mod number_token;
pub mod string_token;
pub mod whitespace_token;

pub enum Tokens {
    Whitespace,
    Null,
    Boolean(bool),
    Number(NumberToken),
    String(StringToken),
    // Object(Object),
    // Array(Array),
}