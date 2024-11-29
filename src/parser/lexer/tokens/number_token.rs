#[derive(Debug)]
pub enum NumberToken {
    Integer(i64),
    Float(f64),
}

const EQ_THRESHOLD: f64 = 0.0001;

impl PartialEq for NumberToken {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Integer(x) => match other {
                Self::Integer(y) => x == y,
                _ => false,
            },
            Self::Float(x) => match other {
                Self::Float(y) => (x - y).abs() < EQ_THRESHOLD,
                _ => false,
            },
        }
    }
}
