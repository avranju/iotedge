use std::collections::HashMap;

use futures::Future;
use hyper::service::Service;
use hyper::{Body, Error as HyperError, Method};

use client::Client;
use error::Error;

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

        self.client.request::<(), String>(
            Method::GET,
            &format!("{}/logs", id),
            Some(query),
            None,
            false,
        )
    }
}
