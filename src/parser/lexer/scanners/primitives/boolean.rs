use crate::parser::buffer::Buffer;
use std::{char, pin::Pin};

const BOOLEAN_TRUE: [char; 3] = ['r', 'u', 'e'];
const BOOLEAN_FALSE: [char; 4] = ['a', 'l', 's', 'e'];

pub fn is_first_char_of_boolean(c: char) -> bool {
    match c {
        't' | 'f' => true,
        _ => false,
    }
}

async fn scan_boolean_true_r(
    buffer: &mut Pin<Box<&mut Buffer>>,
    i: usize
) -> Result<(), &'static str>{
    if i >= BOOLEAN_TRUE.len() {
        return Ok(());
    }

    return match buffer.next_char().await {
        Ok(x) if x == BOOLEAN_TRUE[i] => {
            return Box::pin(scan_boolean_true_r(buffer, i + 1)).await;
        }
        Ok(_) => Err("Unexpected char"),
        Err(x) => Err(x),

    }
}

async fn scan_boolean_false_r(
    buffer: &mut Pin<Box<&mut Buffer>>,
    i: usize
) -> Result<(), &'static str>{
    if i >= BOOLEAN_FALSE.len() {
        return Ok(());
    }

    return match buffer.next_char().await {
        Ok(x) if x == BOOLEAN_FALSE[i] => {
            return Box::pin(scan_boolean_false_r(buffer, i + 1)).await;
        }
        Ok(_) => Err("Unexpected char"),
        Err(x) => Err(x),

    }
}

pub async fn scan_boolean_token(
    first_char: char,
    buffer: &mut Pin<Box<&mut Buffer>>,
) -> Result<bool, &'static str>{
    return match first_char {
        't' if scan_boolean_true_r(buffer, 0).await.is_ok() => Ok(true),
        'f' if scan_boolean_false_r(buffer, 0).await.is_ok() => Ok(false),
        _ => Err("Cannot scan boolean")
    }
}

#[cfg(test)]
mod test_null_primitive {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_first_char_boolean_true() {
        assert!(is_first_char_of_boolean('t'));
    }

    #[test]
    fn test_first_char_boolean_false() {
        assert!(is_first_char_of_boolean('f'));
    }

    #[test]
    fn test_first_char_boolean_invalid() {
        assert!(!is_first_char_of_boolean('q'));
    }

    #[tokio::test]
    async fn test_boolean_scan_true() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "true"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_boolean(buffer_pinned.next_char().await.unwrap()));
        assert!(scan_boolean_token('t', buffer_pinned).await.unwrap());
    }

    #[tokio::test]
    async fn test_boolean_scan_false() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "false"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_boolean(buffer_pinned.next_char().await.unwrap()));
        assert!(!scan_boolean_token('f', buffer_pinned).await.unwrap());
    }

    #[tokio::test]
    async fn test_boolean_scan_error() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "faulty"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_boolean(buffer_pinned.next_char().await.unwrap()));
        assert!(scan_boolean_token('f', buffer_pinned).await.is_err());
    }
}
