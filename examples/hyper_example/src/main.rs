#[macro_use]
extern crate http_router;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate futures;
#[macro_use]
extern crate failure;

mod controller;
mod error;
mod repo;
mod types;
mod utils;

use self::controller::*;
use self::error::{Error, ErrorKind};
use self::repo::{Repo, Transaction, User};
use self::types::ServerFuture;
use self::utils::{read_body, response_with_model};
use failure::{Compat, Fail};
use futures::future;
use futures::prelude::*;
use hyper::rt::{Future, Stream};
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use std::sync::{Arc, Mutex};

type StdFuture = Box<Future<Item = Response<Body>, Error = Compat<Error>> + Send>;

#[derive(Clone)]
struct Application {
    pub repo: Arc<Mutex<Repo>>,
}

impl Service for Application {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Compat<Error>;
    type Future = StdFuture;

    fn call(&mut self, req: Request<Body>) -> StdFuture {
        let repo = self.repo.clone();
        let (req, body) = req.into_parts();
        Box::new(
            read_body(body)
                .and_then(move |body| {
                    let router = router!(
                    GET / => get_users,
                    GET /users => get_users,
                    POST /users => post_users,
                    _ => not_found,
                );

                    let path = req.uri.path();
                    let ctx = Context { repo, body };
                    router(ctx, req.method.into(), path)
                })
                .map_err(|e| e.compat()),
        )
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let json = json!({
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "transactions": [
                    {"hash": "xxx", "value": 12},
                    {"hash": "yyy", "value": 635},
                ],
            },
            {
                "id": 2,
                "name": "Bob",
                "transactions": [
                    {"hash": "zzz", "value": 12},
                ],
            },
        ],
    });

    hyper::rt::run(future::lazy(move || {
        let repo: Repo = serde_json::from_value(json).expect("Failed to parse repo");
        let repo = Arc::new(Mutex::new(repo));
        let app = Application { repo };
        let new_service = move || {
            let res: Result<_, hyper::Error> = Ok(app.clone());
            res
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);
        server
    }));
}
