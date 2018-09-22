use hyper::{Response, Body};
use futures::prelude::*;
use super::error::Error;

pub type ServerFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;
