extern crate http_router;
extern crate hyper;
extern crate futures;
#[macro_use] extern crate serde_json;

use hyper::{Body, Response, Server, Request};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use futures::Future;

type ServerFuture<T> = Box<Future<Item = T, Error = ()>>;

static TEXT: &str = "Hello, World!";

struct Transaction {
    hash: String,
    value: usize,
}

struct User {
    id: usize,
    name: String,
    transactions: Vec<Transaction>,
}

struct Repo {
    users: Vec<User>,
}

fn get_users(_request: Request) -> ServerFuture<Vec<User>> {
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let hash = json!({
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

    let new_svc = || {
        service_fn_ok(|_req|{
            Response::new(Body::from(TEXT))
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
