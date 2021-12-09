use std::fmt::Debug;

use hyper::{Request, Response, Result};
use serde::{de, ser};

use bytes::Bytes;

#[tracing::instrument]
pub fn deserialize<T>(req: Request<Bytes>) -> serde_json::Result<Request<T>>
where
    for<'de> T: de::Deserialize<'de> + Debug,
{
    tracing::info!("deserializing request from json");
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    let json: Request<T> = Request::from_parts(parts, body);
    tracing::info!("finished deserializing request");
    Ok(json)
}

#[tracing::instrument]
pub fn serialize<'a, T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
where
    T: ser::Serialize + Clone + Debug + Into<&'a [u8]>,
{
    tracing::info!("serializing request to json");
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    let json = Request::from_parts(parts, body);
    tracing::info!("finished serializing request",);
    Ok(json)
}

/// Inspects an HTTP `Request`, logs the result and echos back a `Response`
/// An HTTP request has the following format:
///
/// `Method` `Request-URI` `HTTP-Version` `CRLF`
/// `headers` `CRLF`
/// `message-body`
#[tracing::instrument]
pub fn inspector(req: Request<Bytes>) -> Result<Response<()>> {
    tracing::info!("inspecting request");
    let method = req.method();
    let uri = req.uri();
    let version = req.version();
    let headers = req.headers();
    let headers = headers
        .iter()
        .map(|(name, value)| format!("{}:{:?}\n", name, value))
        .collect::<String>();
    tracing::info!(
        "[inspector] {:?} to {:?} ({:?})\n{}",
        method,
        uri,
        version,
        headers
    );
    tracing::info!("finished inspecting request");
    Ok(Response::new(()))
}
