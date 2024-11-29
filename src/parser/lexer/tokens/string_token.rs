use std::string::String;

#[derive(Clone, Debug, PartialEq)]
pub struct StringToken {
    value: Vec<char>,
}

impl StringToken {
    pub fn new() -> Self {
        return StringToken { value: Vec::new() };
    }

    pub fn from(s: &'static str) -> Self {
        return StringToken {
            value: s
                .to_string()
                .chars()
                .into_iter()
                .clone()
                .collect::<Vec<char>>(),
        };
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

    #[test]
    fn test_as_string() {
        let mut string_token = StringToken::new();

        const TEST_DATA: &str = "Hello world!";

        for c in TEST_DATA.chars() {
            string_token.add_char(c);
        }

        let as_string = string_token.as_string();
        assert_eq!(as_string, TEST_DATA);
    }

    #[test]
    fn test_uncode_chars_as_string() {
        let mut string_token = StringToken::new();

        const TEST_DATA: &str = "🤠 Howdy partner 🤠.";

        for c in TEST_DATA.chars() {
            string_token.add_char(c);
        }

        let as_string = string_token.as_string();
        assert_eq!(as_string, TEST_DATA);
    }
}
