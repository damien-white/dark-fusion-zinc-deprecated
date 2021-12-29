use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};

use zinc::logger::initialize_logger;
use zinc::server::{process_request, Database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger().expect("failed to initialize tracing logger");

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let address = match args.first() {
        Some(a) => a,
        None => "127.0.0.1:15550",
    };
    let address = &address.parse::<SocketAddr>()?;

    let listener = TcpListener::bind(&address).await?;
    info!("Listening on: {}", address);

    let initial_db = Database::new();
    initial_db
        .entries
        .lock()
        .insert("foo".to_string(), "bar".to_string());
    let database = Arc::new(initial_db);

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let database = database.clone();

                tokio::spawn(async move { process_request(socket, &database).await });
            }
            Err(err) => error!("error accepting socket; error = {:?}", err),
        }
    }
}
