use crate::{
    utils::{read_payload_content, Error, Payload},
    StatusCode, User,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tauri::command]
pub async fn test_client_connect(
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

    // Write initial connect data
    let payload = Payload(StatusCode::InitialConnect, &user);
    stream.write_all(&payload.get()).await?;

    let (reader, _writer) = stream.split();
    let mut buff_reader = BufReader::new(reader);

    loop {
        let status_code = buff_reader.read_u8().await?;

        let status_code: StatusCode =
            num::FromPrimitive::from_u8(status_code).ok_or(Error::InvalidStatusCode)?;

        match status_code {
            StatusCode::UserConnected => {
                let user = read_payload_content::to_user(&mut buff_reader).await?;

                println!("Client Recived info about new connected user: {:?}", user);
            }
            _ => {}
        }
    }

    Ok(())
}
