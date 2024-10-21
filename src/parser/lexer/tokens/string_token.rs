use std::string::String;

#[derive(Clone)]
pub struct StringToken {
    value: Vec<char>,
}

impl StringToken {
    pub fn new() -> Self {
        return StringToken { value: Vec::new() };
    }

    pub fn as_string(self) -> String {
        self.value.iter().cloned().collect::<String>()
    }

    pub fn add_char(&mut self, c: char) -> &mut StringToken {
        self.value.push(c);
        self
    }
}

#[cfg(test)]
mod test_string_token {
    use super::*;

    const TEST_DATA: &str = "Hello world!";

    #[test]
    fn test_as_string() {
        let mut string_token = StringToken::new();

        for c in TEST_DATA.chars() {
            string_token.add_char(c);
        }

        let as_string = string_token.as_string();
        assert_eq!(as_string, TEST_DATA);
    }
}
