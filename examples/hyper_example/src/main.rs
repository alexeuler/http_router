#[macro_use] 
extern crate http_router;
extern crate hyper;
extern crate serde;
#[macro_use] 
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate futures;

use hyper::{Body, Response, Server, Request};
use hyper::rt::{Future, Stream};
use hyper::service::Service;
use hyper::server::conn::Http;
use std::sync::Arc;
use http_router::*;
use futures::future;

type ServerFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

#[derive(Deserialize)]
struct Transaction {
    hash: String,
    value: usize,
}

#[derive(Deserialize)]
struct User {
    id: usize,
    name: String,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize)]
struct Repo {
    users: Vec<User>,
}

struct Context {
    repo: Arc<Repo>,
}

#[derive(Clone)]
struct Application {
    pub repo: Arc<Repo>,
}

impl Service for Application
{
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = ServerFuture;

    fn call(&mut self, req: Request<Body>) -> ServerFuture {
        let router = router!(
            GET / => get_users,
            _ => not_found,
        );

        let path = req.uri().path();
        let ctx = Context { repo: self.repo.clone() };
        router(ctx, Method::GET, path)
    }
}

fn get_users(context: &Context) -> ServerFuture {
    unimplemented!()
}

fn not_found(context: &Context) -> ServerFuture {
    unimplemented!()
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
        let repo = Arc::new(repo);
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