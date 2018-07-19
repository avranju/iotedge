// Copyright (c) Microsoft. All rights reserved.

use error::Error;
use futures::prelude::*;
use hyper::client::connect::{Connect, Connected, Destination};
use std::path::PathBuf;
use tokio_uds::UnixStream;

mod uri;
pub use self::uri::Uri;

pub struct UnixConnector;

impl Connect for UnixConnector {
    type Transport = UnixStream;
    type Error = Error;
    type Future = ConnectFuture;

    fn connect(&self, dst: Destination) -> Self::Future {
        let state = if dst.scheme() != "unix" {
            ConnectState::Error(Error::InvalidUrlScheme)
        } else {
            Uri::get_uds_path(dst.host())
                .map(|path| ConnectState::Initialized(path))
                .unwrap_or_else(|err| ConnectState::Error(err))
        };

        ConnectFuture::new(state)
    }
}

pub enum ConnectState {
    Initialized(PathBuf),
    // TODO:
    // We are boxing this future because the current version of tokio-uds
    // published on crates.io does not export the ConnectFuture type returned
    // by UnixStream::connect. Once that is exported we can get rid of this
    // box and just use that type.
    Connecting(Box<Future<Item = UnixStream, Error = Error> + Send>),
    Error(Error),
}

pub struct ConnectFuture {
    state: ConnectState,
}

impl ConnectFuture {
    pub fn new(state: ConnectState) -> ConnectFuture {
        match state {
            ConnectState::Initialized(path) => ConnectFuture {
                state: ConnectState::Connecting(Box::new(
                    UnixStream::connect(path).map_err(Error::from),
                )),
            },
            _ => ConnectFuture { state },
        }
    }
}

impl Future for ConnectFuture {
    type Item = (UnixStream, Connected);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.state {
            ConnectState::Initialized(_) => Err(Error::InvalidConnectState),
            ConnectState::Error(ref err) => Err(Error::Connect(format!("{:?}", err))),
            ConnectState::Connecting(ref mut inner) => match inner.poll()? {
                Async::Ready(stream) => Ok(Async::Ready((stream, Connected::new()))),
                Async::NotReady => Ok(Async::NotReady),
            },
        }
    }
}
