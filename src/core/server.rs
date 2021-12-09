use std::net::SocketAddr;
use std::task::{Context, Poll};

use futures::future;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server, StatusCode};

#[derive(Debug)]
pub struct EchoService;

impl Service<Request<Body>> for EchoService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let resp = Response::builder();

        let uri = req.uri();
        tracing::info!("processing request at: {}", uri);
        if uri.path() != "/" {
            let body = Body::from(Vec::new());
            let resp = resp.status(StatusCode::NOT_FOUND).body(body).unwrap();
            tracing::info!("sending response: {:?}", resp);
            return future::ok(resp);
        }

        let body = req.into_body();
        let resp = resp.status(StatusCode::OK).body(body).unwrap();
        tracing::info!("sending response: {:?}", resp);
        future::ok(resp)
    }
}

pub struct MakeSvc;

impl<T> Service<T> for MakeSvc {
    type Response = EchoService;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(EchoService)
    }
}

/// Start listening for incoming client connections
pub async fn listen(addr: &SocketAddr) -> hyper::Result<()> {
    let server = Server::bind(addr).serve(MakeSvc);

    tracing::info!("Listening at: {}", server.local_addr());

    server.await?;

    Ok(())
}

// pub async fn handle_connection(mut stream: TcpStream) -> Result<()> {
//     let mut buffer = [0; 1024];
//
//     let n = stream.read(&mut buffer[..]).await?;
//
//     tracing::info!("Request: {}", String::from_utf8_lossy(&buffer[..n]));
//
//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//
//     stream.write(response.as_bytes()).await?;
//     stream.flush().await?;
//
//     Ok(())
// }
