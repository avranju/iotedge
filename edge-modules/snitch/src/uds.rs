// Copyright (c) Microsoft. All rights reserved.

use std::marker::PhantomData;
use std::path::Path;

use error::Error;
use futures::prelude::*;
use hyper::client::connect::{Connect, Connected, Destination};
use tokio_uds::UnixStream;

pub struct UnixConnector<P>(P);

impl<P> UnixConnector<P> {
    pub fn new(path: P) -> UnixConnector<P> {
        UnixConnector(path)
    }
}

impl<P> Connect for UnixConnector<P>
where
    P: AsRef<Path> + Send + Sync + Clone,
{
    type Transport = UnixStream;
    type Error = Error;
    type Future = ConnectFuture<P>;

    fn connect(&self, _: Destination) -> Self::Future {
        ConnectFuture::new(self.0.clone())
    }
}

pub struct ConnectFuture<P> {
    // TODO:
    // We are boxing this future because the current version of tokio-uds
    // published on crates.io does not export the ConnectFuture type returned
    // by UnixStream::connect. Once that is exported we can get rid of this
    // box and just use that type.
    inner: Box<Future<Item = UnixStream, Error = Error> + Send>,
    _phantom: PhantomData<P>,
}

impl<P> ConnectFuture<P>
where
    P: AsRef<Path>,
{
    pub fn new(path: P) -> ConnectFuture<P> {
        ConnectFuture {
            inner: Box::new(UnixStream::connect(path).map_err(Error::from)),
            _phantom: PhantomData,
        }
    }
}

impl<P> Future for ConnectFuture<P>
where
    P: AsRef<Path>,
{
    type Item = (UnixStream, Connected);
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll()? {
            Async::Ready(stream) => Ok(Async::Ready((stream, Connected::new()))),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}
