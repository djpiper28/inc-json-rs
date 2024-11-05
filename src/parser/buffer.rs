use super::json_path::JsonPath;
use core::panic;
use std::pin::Pin;
use tokio::sync::{Mutex, Semaphore};

pub type BufferChunk = Vec<char>;

/// Stores a buffer of incoming characters as a vector of strings (in `Vec<char>` form).
pub struct Buffer {
    sem: Semaphore,
    data: Mutex<BufferInternalData>,
}

struct BufferInternalData {
    buffers: Vec<BufferChunk>,
    current_buffer_idx: usize,
    current_path: JsonPath,
    /// Whether or not there is more data to be expected after the end of the buffer.
    eof: bool,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            data: Mutex::new(BufferInternalData {
                buffers: Vec::new(),
                current_path: Vec::new(),
                current_buffer_idx: 0,
                eof: false,
            }),
            sem: Semaphore::new(0),
        }
    }

    /// Adds a chunk of data to the buffer
    pub async fn add_data(&mut self, chunk: BufferChunk) -> Result<(), &'static str> {
        let mut data = self.data.lock().await;
        if data.eof {
            return Result::Err("Cannot add data once the EOF has occurred");
        }

        data.buffers.push(chunk);
        self.sem.add_permits(1);
        Result::Ok(())
    }

    /// Called when at the end of the buffer.
    pub async fn eof(&mut self) {
        self.data.lock().await.eof = true;
        self.sem.add_permits(1);
        self.sem.close();
    }

    async fn next_buffer(&mut self) {
        match self.sem.acquire().await {
            Ok(_) => {}
            Err(_) => {
                panic!("Illegal State");
            }
        };

        let mut data = self.data.lock().await;
        data.current_buffer_idx = 0;
        data.buffers.remove(0);
    }

    pub async fn next_char(self: &mut Pin<Box<&mut Self>>) -> Result<char, &'static str> {
        let data = self.data.lock().await;
        if data.eof {
            return Err("EOF reached");
        }

        return match data.buffers.first() {
            Some(buffer) => {
                if data.current_buffer_idx >= buffer.len() {
                    self.next_buffer().await;
                    return Box::pin(self.next_char()).await;
                }

                let c = buffer[data.current_buffer_idx];
                data.current_buffer_idx += 1;
                return Ok(c);
            }
            None => Err("There are no more buffers to get characters from"),
        };
    }
}

#[cfg(test)]
mod test_buffer {
    use std::borrow::BorrowMut;

    use super::*;

    #[tokio::test]
    async fn test_cannot_add_data_after_eof() {
        let mut buffer = Buffer::new();
        buffer.eof().await;

        let err = buffer.add_data(Vec::new()).await;
        assert!(err.is_err(), "Should be in an error state");
    }

    #[tokio::test]
    async fn test_next_char_single_buffer_long() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());
        buffer
            .add_data(vec!['h', 'e', 'l', 'l', 'o'])
            .await
            .unwrap();

        let c1 = buffer.next_char().await.unwrap();
        assert_eq!(c1, 'h');

        let c2 = buffer.next_char().await.unwrap();
        assert_eq!(c2, 'e');
    }

    #[tokio::test]
    async fn test_next_char_many_buffers() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());
        buffer.add_data(vec!['h']).await.unwrap();

        buffer.add_data(vec!['e']).await.unwrap();

        let c1 = buffer.next_char().await.unwrap();
        assert_eq!(c1, 'h');

        let c2 = buffer.next_char().await.unwrap();
        assert_eq!(c2, 'e');
    }

    // #[tokio::test]
    // async fn test_next_char_many_buffers_with_wait() {
    //     let mut buf = Buffer::new();
    //
    //     let mut buffer = Box::pin(buf.borrow_mut());
    //     buffer
    //         .add_data(vec!['h'])
    //         .await
    //         .unwrap();
    //
    //     let mut buffer = Box::pin(buf.borrow_mut());
    //     let c1 = buffer.next_char().await.unwrap();
    //     assert_eq!(c1, 'h');
    //
    //     let c2_future = buffer.next_char();
    //
    //     buffer
    //         .add_data(vec!['e'])
    //         .await
    //         .unwrap();
    //
    //     let c2 = c2_future.await.unwrap();
    //     assert_eq!(c2, 'e');
    // }
}
