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

struct Application {
    repo: Arc<Repo>,
    router: Arc<Fn(Arc<Repo>, Request<Body>, Method, String) -> ServerFuture>,
}

impl Service for Application
{
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = ServerFuture;

    fn call(&mut self, req: Request<Body>) -> ServerFuture {
        (self.router)(self.repo.clone(), req, Method::GET, req.uri().path().to_string())
    }
}

fn get_users(context: Arc<Repo>, _request: Request<Body>) -> ServerFuture {
    unimplemented!()
}

fn not_found(context: Arc<Repo>, _request: Request<Body>) -> ServerFuture {
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

    let repo: Repo = serde_json::from_value(json).expect("Failed to parse repo");
    let repo = Arc::new(repo);

    let router = router!(
        GET / => get_users,
        _ => not_found,
    );
    let router = Arc::new(router);

    let serve = Http::new()
        .serve_addr(&addr, move || {
            let app = Application { repo: repo.clone(), router: router.clone() };
            Ok(app)
        }).unwrap_or_else(|why| {
            eprintln!("Http Server Initialization Error: {}", why);
            std::process::exit(1);
        });

    hyper::rt::spawn(
        serve
            .for_each(move |conn| {
                hyper::rt::spawn(conn.map(|_| ()).map_err(|why| eprintln!("Server Error: {:?}", why)));
                Ok(())
            }).map_err(|_| ())
    );   

    hyper::rt::run(future::empty::<(), ()>());
}
