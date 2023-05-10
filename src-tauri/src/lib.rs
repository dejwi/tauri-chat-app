extern crate num;
#[macro_use]
extern crate num_derive;

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub mod client;
pub mod host;
pub mod utils;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: SocketAddr,
    pub username: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub user: User,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatLogEntry {
    pub message: Option<Message>,
    pub connected: Option<User>,
    pub disconnected: Option<User>,
}

impl ChatLogEntry {
    pub fn connected(user: User) -> Self {
        ChatLogEntry {
            message: None,
            connected: Some(user),
            disconnected: None,
        }
    }
    pub fn disconnected(user: User) -> Self {
        ChatLogEntry {
            message: None,
            connected: None,
            disconnected: Some(user),
        }
    }
    pub fn message(message: Message) -> Self {
        ChatLogEntry {
            message: Some(message),
            connected: None,
            disconnected: None,
        }
    }
}

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum StatusCode {
    Message = 1,
    UserConnected = 2,
    UserDisconnected = 3,
    UserList = 4,
    ChatLog = 5,
    InitialConnect = 6,
}
