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
// use http_router::*;
use futures::future;

type ServerFuture = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

#[derive(Serialize, Deserialize)]
struct Transaction {
    hash: String,
    value: usize,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize)]
struct Repo {
    users: Vec<User>,
}

struct Context {
    pub repo: Arc<Repo>,
    pub body: String,
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
        let repo = self.repo.clone();
        let (req, body) = req.into_parts();
        Box::new(
            read_body(body).and_then(move |body| {
                let router = router!(
                    GET / => get_users,
                    _ => not_found,
                );

                let path = req.uri.path();
                let ctx = Context { repo , body };
                router(ctx, req.method.into(), path)
            })
        )
    }
}

fn get_users(context: &Context) -> ServerFuture {
    let text = serde_json::to_string(&context.repo.users).expect("Failer to serialize json");
    Box::new(
        future::ok(
            Response::builder()
                .status(200)
                .body(text.into())
                .unwrap()
        )
    )
}

fn not_found(context: &Context) -> ServerFuture {
    let text = "Not found";
    Box::new(
        future::ok(
            Response::builder()
                .status(404)
                .body(text.into())
                .unwrap()
        )
    )
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

/// Reads body of request and response in Future format
pub fn read_body(body: hyper::Body) -> impl Future<Item = String, Error = hyper::Error> {
    body.fold(Vec::new(), |mut acc, chunk| {
        acc.extend_from_slice(&*chunk);
        future::ok::<_, hyper::Error>(acc)
    }).map(|bytes| String::from_utf8(bytes).expect("String expected"))
}
