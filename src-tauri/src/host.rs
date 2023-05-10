use std::net::SocketAddr;
use std::sync::Arc;

use crate::{ChatLogEntry, Message, StatusCode, User};

use crate::utils::{payload, Error};
use log::{error, info, warn};
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

#[derive(Clone, Debug)]
pub enum Bcdata {
    Message(Message),
    UserConnected(User),
    UserDisconnected(User),
}

#[tauri::command]
pub async fn host_server(port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(("localhost", port)).await.map_err(|e| {
        error!("Error starting server: {}", e.to_string());
        Error::Connection("Couldn't start server".into())
    })?;

    info!("Started server on: localhost:{port}");

    let (tx, _rx) = broadcast::channel::<Bcdata>(32);
    let online_users = Arc::new(Mutex::new(Vec::<User>::new()));
    let chat_log_entries = Arc::new(Mutex::new(Vec::<ChatLogEntry>::new()));

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("Socket connected: {}", addr);

                let tx = tx.clone();
                let rx = tx.subscribe();
                let online_users = Arc::clone(&online_users);
                let chat_log_entries = Arc::clone(&chat_log_entries);
                tokio::spawn(async move {
                    handle_connection(socket, addr, tx, rx, online_users, chat_log_entries)
                        .await
                        .unwrap_or_else(|err| {
                            error!("{err}");
                        });
                });
            }
            Err(e) => {
                warn!("Error connecting user: {}", e.to_string());
            }
        };
    }
}

/// Handles TCP connection with a socket and data passed from other sockets via broadcast
///
/// Performs a Initial setup
/// 1. Reads streamed User initial data
/// 2. Sends currently online users
/// 3. Sends chatlog
async fn handle_connection(
    mut socket: TcpStream,
    addr: SocketAddr,
    tx: Sender<Bcdata>,
    mut rx: Receiver<Bcdata>,
    online_users: Arc<Mutex<Vec<User>>>,
    chat_log_entries: Arc<Mutex<Vec<ChatLogEntry>>>,
) -> Result<(), Error> {
    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);

    // First read initial user data
    let status_code = reader.read_u8().await?;
    if status_code != StatusCode::InitialConnect as u8 {
        return Err(Error::Connection("Expected a initial connect data".into()));
    }

    let user: User = payload::deserialize(&mut reader).await?;
    tx.send(Bcdata::UserConnected(user.clone())).unwrap();
    online_users.lock().await.push(user.clone());
    chat_log_entries
        .lock()
        .await
        .push(ChatLogEntry::connected(user.clone()));

    // Send currently online users
    writer
        .write_all(&payload::serialize(
            StatusCode::UserList,
            &online_users.lock().await[..],
        ))
        .await?;

    // Send chatlog
    writer
        .write_all(&payload::serialize(
            StatusCode::ChatLog,
            &chat_log_entries.lock().await[..],
        ))
        .await?;

    let mut status_buff = [0u8];
    loop {
        tokio::select! {
        // Handle data received from other sockets
          recived = rx.recv() => {
            match recived.unwrap() {
              Bcdata::UserConnected(user_connected) => {
                if user.id != user_connected.id {
                    let payload = payload::serialize(StatusCode::UserConnected, &user_connected);
                    writer.write_all(&payload).await?;
                }
              }
              Bcdata::UserDisconnected(user_disconnected) => {
                let payload = payload::serialize(StatusCode::UserDisconnected, &user_disconnected);
                writer.write_all(&payload).await?;
              },
              Bcdata::Message(message) => {
                let payload = payload::serialize(StatusCode::Message, &message);
                writer.write_all(&payload).await?;
              },
          }
          }
          // Data streamed from a client
          result = reader.read_exact(&mut status_buff) => {
            if result.unwrap_or(0) == 0 {
              info!("Client disconnected: {}", addr);

              let mut locked_online_users = online_users.lock().await;
              if let Some(pos) = locked_online_users.iter().position(|x| x.id == user.id) {
                locked_online_users.remove(pos);
              }
              chat_log_entries.lock().await.push(ChatLogEntry::disconnected(user.clone()));

              tx.send(Bcdata::UserDisconnected(user)).unwrap();
              break;
            }

            let status_code = num::FromPrimitive::from_u8(status_buff[0]).ok_or(Error::InvalidStatusCode)?;

            match status_code {
                StatusCode::Message => {
                    info!("Server recevied message");
                    let message: Message = payload::deserialize(&mut reader).await?;
                    tx.send(Bcdata::Message(message.clone())).unwrap();
                    chat_log_entries
                        .lock()
                        .await
                        .push(ChatLogEntry::message(message));
                }
                _ => {
                    error!("Client streamed status code that shouldn't be send or is unhandled {:?}", status_code);
                }
            }
          }
        }
    }

    Ok(())
}
