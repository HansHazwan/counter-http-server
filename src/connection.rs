use crate::{
    prelude::*,
    model::*,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use std::{
    sync::Arc,
    fs,
};

pub async fn handle_connection(mut socket: TcpStream, app_state: Arc<AppState>) -> Result<()> {
    let mut buffer = [0; 1024];

    let bytes_read = socket.read(&mut buffer).await?;

    if bytes_read == 0 {
        return Err(Error::ConnectionClosedByPeer);
    }

    let request_line = String::from_utf8_lossy(&buffer[..bytes_read])
        .lines()
        .nth(0)
        .map(String::from)
        .ok_or(Error::HttpRequestIncomplete)?;

    let mut count = app_state.count.lock().await;

    let response = match request_line.as_str() {
        "POST /count HTTP/1.1" => {
            *count += 1;

            String::from("HTTP/1.1 201 SUCCESS")
        },
        "DELETE /count HTTP/1.1" => {
            *count -= 1;

            String::from("HTTP/1.1 201 SUCCESS")
        },
        "GET /count HTTP/1.1" => {
            let counter = CounterObjectTransfer::new(*count);
            let json_content = serde_json::to_string(&counter)?;
            let content_len = json_content.len();

            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
                content_len,
                json_content,
            )
        },
        "GET / HTTP/1.1" => {
            let content = fs::read_to_string("views/index.html")?;
            let content_len = content.len();

            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                content_len,
                content,
            )
        },
        _ => {
            let content = fs::read_to_string("views/404.html")?;
            let content_len = content.len();

            format!(
                "HTTP/1.1 404 NOTFOUND\r\nContent-Length: {}\r\nContent-Type: text/html\r\n{}",
                content_len,
                content,
            )
        },
    };

    socket.write_all(response.as_bytes()).await?;

    Ok(())
}
