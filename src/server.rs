use std::collections::HashMap;
use std::sync::Arc;

use futures_util::SinkExt;
use parking_lot::Mutex;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::error;

/// `Database` structure that holds the central application state
#[derive(Debug, Default)]
pub struct Database {
    pub entries: Mutex<HashMap<String, String>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }
}

/// Possible requests our clients can send us
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
}

impl Request {
    fn parse(input: &str) -> Result<Request, String> {
        tracing::info!("parsing request");
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET request requires a key")?;
                if parts.next().is_some() {
                    return Err("GET request contains extra data".into());
                }
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("SET operation requires a key".into()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("SET operation requires a value".into()),
                };
                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => Err(format!("unknown command: {}", cmd)),
            None => Err("empty input".into()),
        }
    }
}

/// Responses to the `Request` commands above
pub enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        message: String,
    },
}

impl Response {
    fn serialize(&self) -> String {
        match self {
            Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { message: ref msg } => format!("error: {}", msg),
        }
    }
}

pub async fn process_request(socket: TcpStream, database: &Arc<Database>) {
    let mut lines = Framed::new(socket, LinesCodec::new());

    while let Some(result) = lines.next().await {
        match result {
            Ok(line) => {
                let response = handle_request(&line, database);
                let serialized = response.serialize();

                if let Err(err) = lines.send(serialized.as_str()).await {
                    error!("error on sending response; error = {:?}", err);
                }
            }
            Err(err) => {
                error!("error on decoding from socket; error = {:?}", err);
            }
        }
    }
}

pub fn handle_request(line: &str, database: &Arc<Database>) -> Response {
    let request = match Request::parse(line) {
        Ok(req) => req,
        Err(err) => return Response::Error { message: err },
    };

    let mut database = database.entries.lock();
    match request {
        Request::Get { key } => match database.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.clone(),
            },
            None => Response::Error {
                message: format!("no key {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = database.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}
