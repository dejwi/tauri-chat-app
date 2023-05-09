//! Helps parsing data into bytes for passing in socket
//!
//! serialize - `(1 byte - status code)(4 bytes - content length)(content)`
//!
//! deserialize - `(4 bytes - content length)(content)`

use tokio::{
    io::{AsyncBufRead, AsyncReadExt, BufReader},
    net::tcp::ReadHalf,
};

use crate::utils::Error;
use crate::StatusCode;

/// Serializes data
///
/// format - `(1 byte - status code)(4 bytes - content length)(content)`
pub fn serialize<S>(status_code: StatusCode, data: &S) -> Vec<u8>
where
    S: serde::Serialize,
{
    let data_bytes = bincode::serialize(data).unwrap();

    let content_len = data_bytes.len() as u32;
    let mut serialized: Vec<u8> = Vec::with_capacity(content_len as usize + 5);

    serialized.push(status_code as u8);
    serialized.extend(content_len.to_be_bytes());
    serialized.extend(data_bytes);

    serialized
}

/// Assumes status code is already extracted
///
/// Reads stream - `(content-length: 4 bytes)(content)`
pub async fn deserialize_to_bytes<T>(buff_reader: &mut T) -> Result<Vec<u8>, Error>
where
    T: AsyncBufRead + std::marker::Unpin,
{
    let content_len = buff_reader.read_u32().await?;

    let mut content_buff: Vec<u8> = vec![0; content_len as usize];
    buff_reader.read_exact(&mut content_buff).await?;

    Ok(content_buff)
}

/// Assumes status code is already extracted
///
/// Reads stream - `(content-length: 4 bytes)(content)`
pub async fn deserialize<T, U>(buff_reader: &mut T) -> Result<U, Error>
where
    U: serde::de::DeserializeOwned,
    T: AsyncBufRead + std::marker::Unpin,
{
    let data_bytes = deserialize_to_bytes(buff_reader).await?;

    let data: U = bincode::deserialize(&data_bytes)?;

    Ok(data)
}
