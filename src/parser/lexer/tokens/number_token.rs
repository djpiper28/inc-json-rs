#[derive(Debug)]
pub enum NumberToken {
    Integer(i64),
    Float(f64),
}

impl PartialEq for NumberToken {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(x) => match other {
                Self::Integer(y) => x == y,
                _ => false,
            },
            _ => false,
        }
    }
}
