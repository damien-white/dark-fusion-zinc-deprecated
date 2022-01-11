use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

use zinc::client::Client;
use zinc::logger::initialize_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger().expect("failed to initialize trace logger");

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let address = match args.first() {
        Some(a) => a,
        None => "127.0.0.1:15550",
    };
    let address = &address.parse()?;

    let stdin = FramedRead::new(tokio::io::stdin(), BytesCodec::new());
    let stdin = stdin.map(|b| b.map(|bytes| bytes.freeze()));
    let stdout = FramedWrite::new(tokio::io::stdout(), BytesCodec::new());

    Client::init(address, stdin, stdout).await?;

    Ok(())
}
