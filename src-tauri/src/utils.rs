use tokio::{
    io::{AsyncReadExt, BufReader},
    net::tcp::ReadHalf,
};

use crate::{StatusCode, User};

mod error;
pub use error::Error;

/// Help parsing data into bytes for passing in socket
///
/// format - `(1 byte - status code)(4 bytes - content length)(content)`
pub struct Payload<T>(pub StatusCode, pub T);

impl<'a> Payload<&'a User> {
    /// Converts Payload with `User` into bytes
    pub fn get(&self) -> Vec<u8> {
        let Payload(status_code, user) = self;
        let user_bytes = bincode::serialize(user).unwrap();

        let content_len = user_bytes.len() as u32;
        let mut data: Vec<u8> = Vec::with_capacity(content_len as usize + 5);

        data.push(*status_code as u8);
        data.extend(content_len.to_be_bytes());
        data.extend(user_bytes);

        data
    }
}

/// Fns for easier payload reading
///
/// Assumes status code is already extracted
///
/// Reads stream - `(content-length: 4 bytes)(content)`
pub mod read_payload_content {
    use super::*;
    type ReadResult<T> = Result<T, Error>;

    pub async fn to_bytes(buff_reader: &mut BufReader<ReadHalf<'_>>) -> ReadResult<Vec<u8>> {
        let content_len = buff_reader.read_u32().await?;

        let mut content_buff: Vec<u8> = vec![0; content_len as usize];
        buff_reader.read_exact(&mut content_buff).await?;

        Ok(content_buff)
    }

    pub async fn to_user(buff_reader: &mut BufReader<ReadHalf<'_>>) -> ReadResult<User> {
        let user_bytes = to_bytes(buff_reader).await?;

        let user: User = bincode::deserialize(&user_bytes)?;

        Ok(user)
    }
}
