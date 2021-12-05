use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

use zinc::config::Config;
use zinc::telemetry::init_tracing;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_tracing("debug").expect("Failed to initialize trace logger.");

    let config = Config::new().expect("Failed to load config file.");
    let addr = format!("{}:{}", &config.app.hostname, &config.app.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on: {}", listener.local_addr()?);

    // Start loop so that the server can begin accepting client connections
    loop {
        match listener.accept().await {
            Ok((stream, socket_addr)) => {
                info!("Connection from: {:?}", socket_addr);
                handle_connection(stream).await?;
            }
            Err(e) => {
                error!("Failed connection attempt: {:?}", e);
            }
        }
    }
}

/// Handle a single client connection
pub async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer[..]).await?;

    info!("Request: {}", String::from_utf8_lossy(&buffer[..n]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
