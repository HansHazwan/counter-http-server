use tokio::{
    net::{TcpStream, TcpListener},
    io::{AsyncWriteExt, AsyncReadExt},
    sync::Mutex,
};
use std::sync::Arc;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on http://localhost:8080");

    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New Connection: {:?}", addr);

        let counter = Arc::clone(&counter);

        tokio::spawn(async move {
            match handle_connection(socket, counter).await {
                Ok(_) => {},
                Err(err) => {
                    log::error!("{}", err);
                },
            }
        });
    }

    Ok(())
}

async fn handle_connection(mut socket: TcpStream, counter: Arc<Mutex<i32>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let bytes_read = socket.read(&mut buffer).await.unwrap();

    if bytes_read == 0 {
        return Err("Connection closed by peer.".into());
    }

    let request_line = String::from_utf8_lossy(&buffer[..bytes_read])
        .lines()
        .nth(0)
        .map(String::from)
        .ok_or::<Box<dyn std::error::Error>>("Http Request is empty.".into())?;

    let mut count = counter.lock().await;
    let mut response = String::new();

    match request_line.as_str() {
        "POST /count HTTP/1.1" => {
            *count += 1;
            response = String::from("HTTP/1.1 201 SUCCESS");
        },
        "DELETE /count HTTP/1.1" => {
            *count -= 1;
            response = String::from("HTTP/1.1 201 SUCCESS");
        },
        "GET / HTTP/1.1" => {
            let content = fs::read_to_string("views/index.html")?;
            let content_len = content.len();

            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                content_len,
                content,
            );
        },
        "GET /count HTTP/1.1" => {
            let content = format!("{{\n\t\"count\": {}\n}}", count);
            let content_len = content.len();

            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                content_len,
                content,
            );
        },
        _ => {
            let content = fs::read_to_string("views/404.html")?;
            let content_len = content.len();

            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                content_len,
                content,
            );
        }
    }

    socket.write_all(response.as_bytes()).await?;
    Ok(())
}
