use std::net::SocketAddr;

use bytes::Bytes;
use futures::{future, Sink, SinkExt};
use tokio::net::TcpStream;
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};

#[derive(Debug)]
pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub async fn init<I, O>(
        address: &SocketAddr,
        mut stdin: I,
        mut stdout: O,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        I: Stream<Item = Result<Bytes, std::io::Error>> + Unpin,
        O: Sink<Bytes, Error = std::io::Error> + Unpin,
    {
        let mut stream = TcpStream::connect(address).await?;

        let (reader, writer) = stream.split();
        let mut sink = FramedWrite::new(writer, BytesCodec::new());

        let mut stream =
            FramedRead::new(reader, BytesCodec::new()).map(|b| b.map(|bytes| bytes.freeze()));

        match future::join(sink.send_all(&mut stdin), stdout.send_all(&mut stream)).await {
            (Err(err), _) | (_, Err(err)) => Err(err.into()),
            _ => Ok(()),
        }
    }
}
