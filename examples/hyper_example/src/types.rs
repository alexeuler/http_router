use super::error::Error;
use futures::prelude::*;
use hyper::{Body, Response};

pub type ServerFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;
