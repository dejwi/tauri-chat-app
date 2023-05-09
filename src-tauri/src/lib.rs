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

#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone)]
pub enum StatusCode {
    Message = 1,
    UserConnected = 2,
    UserDisconnected = 3,
    UserList = 4,
    AllMessages = 5,
    InitialConnect = 6,
}
