use std::net::SocketAddr;

use crate::{StatusCode, User};

use crate::utils::{read_payload_content, Payload};
use tauri::http::status;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpListener, TcpStream,
    },
    sync::broadcast::{self, Receiver, Sender},
};

#[derive(Clone, Debug)]
pub enum Bcdata {
    Message(User, String),
    UserConnected(User),
    UserDisconnected(User),
    UserList(Vec<User>),
    AllMessages(Vec<(String, User)>),
}

impl Bcdata {
    fn get_status_code(&self) -> StatusCode {
        match self {
            Bcdata::Message(_, _) => StatusCode::Message,
            Bcdata::AllMessages(_) => StatusCode::AllMessages,
            Bcdata::UserConnected(_) => StatusCode::UserConnected,
            Bcdata::UserDisconnected(_) => StatusCode::UserDisconnected,
            Bcdata::UserList(_) => StatusCode::UserList,
        }
    }
}

#[tauri::command]
pub async fn host_server(port: u16) -> Result<(), String> {
    let listener = TcpListener::bind(("localhost", port)).await.map_err(|e| {
        eprintln!("Error starting server: {}", e.to_string());
        "Couldn't start server"
    })?;

    println!("Started server on: localhost:{port}");

    let (tx, _rx) = broadcast::channel::<Bcdata>(32);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Socket connected: {}", addr);

                let tx = tx.clone();
                let rx = tx.subscribe();
                tokio::spawn(async move {
                    handle_connection(socket, addr, tx, rx).await;
                });
            }
            Err(e) => {
                eprintln!("Error connecting user: {}", e.to_string());
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
) {
    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut status_buff = [0u8];

    let status_code = reader.read_u8().await.unwrap();
    if status_code != StatusCode::InitialConnect as u8 {
        panic!("Expected a initial connect data");
    }

    let user = read_payload_content::to_user(&mut reader).await;
    
    println!("Deserialized connected user: {:?}", user);
    tx.send(Bcdata::UserConnected(user.clone())).unwrap();

    loop {
        tokio::select! {
          recived = rx.recv() => {
            match recived.unwrap() {
              Bcdata::UserConnected(user) => {
                  let payload = Payload(StatusCode::UserConnected, &user);
                  writer.write_all(&payload.get()).await.unwrap();
              }
              Bcdata::UserDisconnected(user) => {
                let payload = Payload(StatusCode::UserDisconnected, &user);
                writer.write_all(&payload.get()).await.unwrap();
              }
              _ => (),
          }
          }
          result = reader.read_exact(&mut status_buff) => {
            if result.unwrap_or(0) == 0 {
              println!("Client disconnected: {}", addr);
              tx.send(Bcdata::UserDisconnected(user)).unwrap();
              break;
            }

            let status_code = status_buff[0];
            println!("Got status code: {}",status_code );

            handle_data_stream(num::FromPrimitive::from_u8(status_code), &tx, &mut reader ).await.unwrap();
          }
        }
    }
}

/// Handle data streamed from a client
async fn handle_data_stream<'a>(
    status_code: Option<StatusCode>,
    tx: &Sender<Bcdata>,
    buff_reader: &mut BufReader<ReadHalf<'_>>,
) -> Result<(), &'static str> {
    let status_code = status_code.ok_or("Invalid status code")?;

    use StatusCode::*;
    match status_code {
        UserConnected => {
            // clients shouldn't send this
        }
        _ => (),
    }

    Ok(())
}
