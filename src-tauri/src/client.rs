use std::sync::{Arc, Mutex};

use crate::{
    utils::{payload, Error},
    ChatLogEntry, Message, StatusCode, User,
};
use log::{error, info};
use tauri::{Manager, Window};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tauri::command]
pub async fn client_connect(
    window: Window,
    port: u16,
    username: String,
    avatar_url: String,
) -> Result<(), Error> {
    let mut stream = TcpStream::connect(("localhost", port))
        .await
        .map_err(|_| Error::Connection("Couldn't connect to the server".into()))?;

    let user = User {
        id: stream.local_addr().unwrap(),
        username,
        avatar_url,
    };

    // send id to front
    window.emit_all("client-id", user.id).unwrap();

    // Write initial connect data
    let init_bytes = payload::serialize(StatusCode::InitialConnect, &user);
    stream.write_all(&init_bytes).await?;

    let (reader, writer) = stream.into_split();
    let writer = Arc::new(Mutex::new(writer));
    let mut buff_reader = BufReader::new(reader);

    let writer2 = Arc::clone(&writer);
    window.listen("send-message", move |event| {
        let user = user.clone();
        let mut writer = writer2.lock().unwrap();

        // goddamn
        tokio::task::block_in_place(move || {
            tauri::async_runtime::handle().block_on(async {
                if let Some(content) = event.payload() {
                    let content = content.trim().trim_matches(|c| c == '"');
                    info!("Recived send-message event with payload: {content}");
                    let message = Message {
                        user,
                        content: content.into(),
                    };
                    let data_bytes = payload::serialize(StatusCode::Message, &message);
                    writer.write_all(&data_bytes).await.unwrap();
                }
            });
        });
    });

    loop {
        let status_code = buff_reader.read_u8().await?;

        let status_code: StatusCode =
            num::FromPrimitive::from_u8(status_code).ok_or(Error::InvalidStatusCode)?;

        match status_code {
            StatusCode::UserConnected => {
                let user: User = payload::deserialize(&mut buff_reader).await?;

                info!("Received user connected: {:?}", user.username);
                window.emit_all("user-connected", user).unwrap();
            }
            StatusCode::UserDisconnected => {
                let user: User = payload::deserialize(&mut buff_reader).await?;

                info!("Received user connected: {:?}", user.username);
                window.emit_all("user-disconnected", user).unwrap();
            }
            StatusCode::Message => {
                let message: Message = payload::deserialize(&mut buff_reader).await?;

                info!("Client received message: {}", message.content);
                window.emit_all("received-message", message).unwrap();
            }
            StatusCode::UserList => {
                let users: Vec<User> = payload::deserialize(&mut buff_reader).await?;

                info!("Received info about online users");
                window.emit_all("set-online-users", users).unwrap();
            }
            StatusCode::ChatLog => {
                let chat_log: Vec<ChatLogEntry> = payload::deserialize(&mut buff_reader).await?;

                info!("Received chat log");
                window.emit_all("set-chat-log", chat_log).unwrap();
            }
            _ => {
                error!(
                    "Server streamed status code that shouldn't be send or is unhandled {:?}",
                    status_code
                );
            }
        }
    }
}
