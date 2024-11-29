use crate::parser::buffer::Buffer;
use std::{char, pin::Pin};

// The 'n' has been scanned
const NULL: [char; 3] = ['u', 'l', 'l'];

pub fn is_first_char_of_null(c: char) -> bool {
    match c {
        'n' => true,
        _ => false,
    }
}

async fn scan_null_token_r(
    buffer: &mut Pin<Box<&mut Buffer>>,
    i: usize
) -> Result<(), &'static str>{
    if i >= NULL.len() {
        return Ok(());
    }

    return match buffer.next_char().await {
        Ok(x) if x == NULL[i] => {
            return Box::pin(scan_null_token_r(buffer, i + 1)).await;
        }
        Ok(_) => Err("Unexpected char"),
        Err(x) => Err(x),

    }
}

pub async fn scan_null_token(
    buffer: &mut Pin<Box<&mut Buffer>>,
) -> Result<(), &'static str>{
    return scan_null_token_r(buffer, 0).await;
}

#[cfg(test)]
mod test_null_primitive {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_null_is_first_char_of_null_is_true() {
        assert!(is_first_char_of_null('n'));
    }

    #[test]
    fn test_null_is_invalid() {
        assert!(!is_first_char_of_null('u'));
    }

    #[tokio::test]
    async fn test_null_scan() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "null"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_null(buffer_pinned.next_char().await.unwrap()));
        assert!(scan_null_token(buffer_pinned).await.is_ok());
    }

    #[tokio::test]
    async fn test_null_scan_failure() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "noob"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_null(buffer_pinned.next_char().await.unwrap()));
        assert!(scan_null_token(buffer_pinned).await.is_err());
    }

    #[tokio::test]
    async fn test_null_scan_failure_2() {
        let mut buffer = Buffer::new();

        assert!(buffer
            .add_data(
                "nuLL"
                    .to_string()
                    .chars()
                    .into_iter()
                    .clone()
                    .collect::<Vec<char>>()
            )
            .await
            .is_ok());

        let buffer_pinned = &mut Box::pin(buffer.borrow_mut());
        assert!(is_first_char_of_null(buffer_pinned.next_char().await.unwrap()));
        assert!(scan_null_token(buffer_pinned).await.is_err());
    }
}
