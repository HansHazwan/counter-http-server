use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::Mutex;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on http://localhost:8080");

    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New Connection: {:?}", addr);

        let counter = Arc::clone(&counter);

        tokio::spawn(async move {
            handle_connection(socket, counter).await;
        });
    }

    Ok(())
}

async fn handle_connection(mut socket: TcpStream, counter: Arc<Mutex<i32>>) {
    let mut buffer = [0; 1024];

    if let Ok(bytes_read) = socket.read(&mut buffer).await {
        if bytes_read == 0 {
            println!("Connection closed by peer.");
        }

        let http_request = String::from_utf8_lossy(&buffer[..bytes_read])
            .lines()
            .map(String::from)
            .collect::<Vec<String>>();

        if let Some(request_line) = http_request.get(0) {
            match request_line.as_str() {
                "GET /increment HTTP/1.1" => {
                    let mut count = counter.lock().await;
                    *count += 1;

                    socket.write_all(b"HTTP/1.1 201 Success").await.unwrap();
                },
                "GET /decrement HTTP/1.1" => {
                    let mut count = counter.lock().await;
                    *count -= 1;

                    socket.write_all(b"HTTP/1.1 201 Success").await.unwrap();
                },
                "GET / HTTP/1.1" => {
                    let count = counter.lock().await;

                    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\n{}", count);

                    socket.write_all(response.as_bytes()).await.unwrap();
                },
                _ => {
                    socket.write_all(b"HTTP/1.1 404 Not Found\r\n").await.unwrap();
                }
            }
        } else {
            println!("The http request is empty.");
        }
    } else {
        println!("Error to read socket.");
    }
}
