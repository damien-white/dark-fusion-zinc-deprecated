use hyper::Server;
use tracing::info;

use zinc::config::Config;
use zinc::core::server::MakeSvc;
use zinc::error::Result;
use zinc::telemetry::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing("trace").expect("failed to initialize tracing logger");

    let config = Config::new().expect("failed to load config file");
    let addr = &config.bind_address();

    let server = Server::bind(addr).serve(MakeSvc);
    info!("Listening at: {}", server.local_addr());
    server.await?;

    Ok(())
}
