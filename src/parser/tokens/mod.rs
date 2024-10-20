use number::Number;

pub mod number;
pub mod whitespace;

pub enum Tokens {
    Whitespace,
    Null,
    Boolean(bool),
    Number(Number),
    // String(String),
    // Object(Object),
    // Array(Array),
}
