mod error;
mod prelude;
mod connection;
mod model;

use crate::{
    prelude::*,
    model::AppState,
    connection::handle_connection,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    log::info!("Server running on http://localhost:8080");

    let app_state = Arc::new(AppState::new("Counter App"));

    loop {
        let (socket, addr) = listener.accept().await?;
        log::info!("New Connection: {:?}", addr);

        let app_state = Arc::clone(&app_state);

        tokio::spawn(async move {
            match handle_connection(socket, app_state).await {
                Ok(_) => {
                    log::info!("Successful response.");
                },
                Err(e) => {
                    log::error!("{}", e);
                },
            }
        });
    }

    Ok(())
}


