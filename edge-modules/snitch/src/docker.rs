use std::collections::HashMap;
use std::str;

use bytes::{Buf, IntoBuf};
use futures::{future, Future};
use hyper::service::Service;
use hyper::{Body, Error as HyperError, Method};

use client::Client;
use error::{Error, Result};

pub struct DockerClient<S>
where
    S: 'static + Service<ReqBody = Body, ResBody = Body, Error = HyperError>,
{
    client: Client<S>,
}

impl<S> DockerClient<S>
where
    S: 'static + Service<ReqBody = Body, ResBody = Body, Error = HyperError>,
{
    pub fn new(client: Client<S>) -> DockerClient<S> {
        DockerClient { client }
    }

    pub fn logs(&self, id: &str) -> impl Future<Item = Option<String>, Error = Error> {
        let mut query = HashMap::new();
        query.insert("stdout", "true");
        query.insert("stderr", "true");

        self.client
            .request_bytes::<()>(
                Method::GET,
                &format!("containers/{}/logs", id),
                Some(query),
                None,
                false,
            )
            .and_then(|bytes| {
                bytes
                    .map(|bytes| {
                        let mut buf = bytes.into_buf();
                        let mut logs = String::new();
                        while buf.has_remaining() {
                            let line = read_line(&mut buf);
                            if let Err(err) = line {
                                return future::err(err);
                            }
                            let line = line.expect("Unexpected error value in result");
                            logs.push_str(&line);
                        }

                        future::ok(Some(logs))
                    })
                    .unwrap_or(future::ok(None))
            })
    }
}

/// Logs parser
/// Logs are emitted with a simple header to specify stdout or stderr
///
/// 01 00 00 00 00 00 00 1f 52 6f 73 65 73 20 61 72  65 ...
/// │  ─────┬── ─────┬─────  R  o  s  e  s     a  r   e ...
/// │       │        │
/// └stdout │        │
///         │        └ 0x0000001f = log message is 31 bytes
///       unused
///
fn read_line<T: Buf>(buf: &mut T) -> Result<String> {
    buf.advance(4); // ignore stream type & unused bytes
    let len = buf.get_u32_be() as usize; // read length
    let result = {
        let bytes = buf.bytes();

        // utf8 decode log line
        str::from_utf8(&bytes[..len])
            .map_err(Error::from)
            // TODO: I am getting a strange lifetime error if I attempt to return
            // a &str instead of a String. Since buf's lifetime exceeds the lifetime
            // of this function, it *should* be possible to return a slice that points
            // inside buf but the compiler doesn't like it for some reason.
            .map(|s| s.to_owned())
    };

    if result.is_ok() {
        buf.advance(len); // advance buffer by log text we just read
    }

    result
}
