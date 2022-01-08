use std::net::{AddrParseError, SocketAddr};
use std::sync::Arc;

use tokio::net::TcpListener;
use tracing::{debug_span, error, info, info_span};

use zinc::server::{process_request, Database};
use zinc::tracer::init_trace_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_trace_logger();

    let outer_span = info_span!("outer", level = 0);
    let _outer_span_guard = outer_span.enter();

    let inner_span = debug_span!("inner", level = 1);
    let _inner_guard = inner_span.enter();

    // Log something simple; creates an "event"
    info!(a_bool = true, answer = 42, message = "first example");

    let address = outer_span.in_scope(|| -> Result<SocketAddr, AddrParseError> {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        let address = match args.first() {
            Some(val) => val,
            None => "127.0.0.1:15550",
        };
        address.parse::<SocketAddr>()
    })?;

    // let args = std::env::args().skip(1).collect::<Vec<_>>();
    // let address = match args.first() {
    //     Some(val) => val,
    //     None => "127.0.0.1:15550",
    // };
    // let address = &address.parse::<SocketAddr>()?;

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
