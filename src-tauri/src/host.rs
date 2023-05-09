use std::net::SocketAddr;

use crate::{Message, StatusCode, User};

use crate::utils::{payload, Error};
use log::{error, info, warn};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

#[derive(Clone, Debug)]
pub enum Bcdata {
    Message(Message),
    UserConnected(User),
    UserDisconnected(User),
    UserList(Vec<User>),
    AllMessages(Vec<(String, User)>),
}

impl Bcdata {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Bcdata::Message(_) => StatusCode::Message,
            Bcdata::AllMessages(_) => StatusCode::AllMessages,
            Bcdata::UserConnected(_) => StatusCode::UserConnected,
            Bcdata::UserDisconnected(_) => StatusCode::UserDisconnected,
            Bcdata::UserList(_) => StatusCode::UserList,
        }
    }
}

#[tauri::command]
pub async fn host_server(port: u16) -> Result<(), Error> {
    let listener = TcpListener::bind(("localhost", port)).await.map_err(|e| {
        error!("Error starting server: {}", e.to_string());
        Error::Connection("Couldn't start server".into())
    })?;

    info!("Started server on: localhost:{port}");

    let (tx, _rx) = broadcast::channel::<Bcdata>(32);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("Socket connected: {}", addr);

                let tx = tx.clone();
                let rx = tx.subscribe();
                tokio::spawn(async move {
                    handle_connection(socket, addr, tx, rx)
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

    Ok(())
}

async fn handle_connection(
    mut socket: TcpStream,
    addr: SocketAddr,
    tx: Sender<Bcdata>,
    mut rx: Receiver<Bcdata>,
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

    let mut status_buff = [0u8];
    loop {
        tokio::select! {
        // Handle data received from other socket
          recived = rx.recv() => {
            match recived.unwrap() {
              Bcdata::UserConnected(user) => {
                  let payload = payload::serialize(StatusCode::UserConnected, &user);
                  writer.write_all(&payload).await?;
              }
              Bcdata::UserDisconnected(user) => {
                let payload = payload::serialize(StatusCode::UserDisconnected, &user);
                writer.write_all(&payload).await?;
              },
              Bcdata::Message(message) => {
                let payload = payload::serialize(StatusCode::Message, &message);
                writer.write_all(&payload).await?;
              },
              _ => (),
          }
          }
          result = reader.read_exact(&mut status_buff) => {
            if result.unwrap_or(0) == 0 {
              info!("Client disconnected: {}", addr);
              tx.send(Bcdata::UserDisconnected(user)).unwrap();
              break;
            }

            let status_code = status_buff[0];
            handle_client_stream(num::FromPrimitive::from_u8(status_code), &tx, &mut reader ).await?;
          }
        }
    }

    Ok(())
}

/// Handles data streamed from a client
async fn handle_client_stream<'a>(
    status_code: Option<StatusCode>,
    tx: &Sender<Bcdata>,
    buff_reader: &mut BufReader<ReadHalf<'_>>,
) -> Result<(), Error> {
    let status_code = status_code.ok_or(Error::InvalidStatusCode)?;

    match status_code {
        StatusCode::UserConnected => {
            // clients shouldn't send this
        }
        StatusCode::Message => {
            info!("Server recevied message");
            let data: Message = payload::deserialize(buff_reader).await?;
            tx.send(Bcdata::Message(data)).unwrap();
        }
        _ => (),
    }

    Ok(())
}
