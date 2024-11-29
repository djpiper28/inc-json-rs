use std::pin::Pin;
use tokio::sync::{Mutex, Semaphore};

/// The buffer reads chunks of data at a time and adds it to an internal queue.
pub type BufferChunk = Vec<char>;

/// Stores a buffer of incoming characters as a vector of strings (in `Vec<char>` form).
pub struct Buffer {
    sem: Semaphore,
    data: Mutex<BufferInternalData>,
}

struct BufferInternalData {
    buffers: Vec<BufferChunk>,
    current_buffer_idx: usize,
    /// Whether or not there is more data to be expected after the end of the buffer.
    eof: bool,
}

impl Buffer {
    pub fn new() -> Self {
        println!("uwu");
        Buffer {
            data: Mutex::new(BufferInternalData {
                buffers: Vec::new(),
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
    }

    pub async fn replace_char(&mut self, c: char) {
        let mut data = self.data.lock().await;
        let mut new_buffer = Vec::new();
        new_buffer.push(c);

        if !data.buffers.is_empty() {
            let mut i = data.current_buffer_idx;
            while i < data.buffers[0].len() {
                new_buffer.push(data.buffers[0][i]);

                i += 1
            }

            data.buffers.remove(0);
        }

        data.buffers.insert(0, new_buffer);
        data.current_buffer_idx = 0;
        self.sem.add_permits(1);
    }

    pub async fn next_char(self: &mut Pin<Box<&mut Self>>) -> Result<char, &'static str> {
        loop {
            let mut data = self.data.lock().await;
            let buffer = data.buffers.first();
            let at_end_of_current_buffer = match buffer {
                Some(b) => data.current_buffer_idx >= b.len(),
                None => true,
            };

            if at_end_of_current_buffer {
                if data.eof && data.buffers.is_empty() {
                    return Err("EOF reached");
                }

                // Drop to prevent dead-lock
                drop(data);
                if self.sem.acquire().await.is_err() {
                    return Err("Cannot unlock semaphore - EOF probably");
                }

                let mut data = self.data.lock().await;
                // The first buffer is the one that has been read fully
                if data.eof && data.buffers.len() <= 1 {
                    return Err("EOF reached");
                }

                data.current_buffer_idx = 0;
                data.buffers.remove(0);
            } else {
                let c = buffer.unwrap()[data.current_buffer_idx];
                data.current_buffer_idx += 1;
                return Ok(c);
            }
        }
    }
}

#[cfg(test)]
mod test_buffer {
    use super::*;
    use std::borrow::BorrowMut;

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
    async fn test_next_char_single_with_replacement() {
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

        buffer.replace_char('e').await;

        let c3 = buffer.next_char().await.unwrap();
        assert_eq!(c3, 'e');

        let c4 = buffer.next_char().await.unwrap();
        assert_eq!(c4, 'l');
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

    #[tokio::test]
    async fn test_next_char_after_eof_errors() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());
        buffer.add_data(vec!['h']).await.unwrap();

        buffer.add_data(vec!['e']).await.unwrap();

        let c1 = buffer.next_char().await.unwrap();
        assert_eq!(c1, 'h');

        let c2 = buffer.next_char().await.unwrap();
        assert_eq!(c2, 'e');

        buffer.eof().await;

        assert!(buffer.next_char().await.is_err());
    }

    #[tokio::test]
    async fn test_next_char_after_eof_errors_with_a_queue() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());
        buffer.add_data(vec!['h']).await.unwrap();
        buffer.add_data(vec!['e']).await.unwrap();
        buffer.eof().await;

        let c1 = buffer.next_char().await.unwrap();
        assert_eq!(c1, 'h');

        let c2 = buffer.next_char().await.unwrap();
        assert_eq!(c2, 'e');

        assert!(buffer.next_char().await.is_err());
    }

    #[tokio::test]
    async fn test_next_char_after_eof_errors_with_no_data() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());
        buffer.eof().await;
        assert!(buffer.next_char().await.is_err());
    }

    #[tokio::test]
    async fn test_replacement_empty_queue() {
        let mut buf = Buffer::new();
        let mut buffer = Box::pin(buf.borrow_mut());

        buffer.replace_char('a').await;

        let c1 = buffer.next_char().await;
        assert_eq!(c1.unwrap(), 'a');
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
