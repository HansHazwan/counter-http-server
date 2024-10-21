use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Custom Error: {0}")]
    Custom(String),

    #[error("I/O Error: {0}")]
    Io(#[from] tokio::io::Error),

    #[error("Serde Json Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Connection closed by peer.")]
    ConnectionClosedByPeer,

    #[error("Http Request Incomplete.")]
    HttpRequestIncomplete,
}
