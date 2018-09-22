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

mod error;

use self::error::{Error, ErrorKind};
use failure::{Compat, Fail};
use futures::future;
use futures::prelude::*;
use hyper::rt::{Future, Stream};
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use serde::Serialize;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

type ServerFuture = Box<Future<Item = Response<Body>, Error = Error> + Send>;
type StdFuture = Box<Future<Item = Response<Body>, Error = Compat<Error>> + Send>;

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    hash: String,
    value: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    users: Vec<User>,
}

struct Context {
    pub repo: Arc<Mutex<Repo>>,
    pub body: String,
}

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

fn get_users(context: &Context) -> ServerFuture {
    let repo = context.repo.lock().expect("Failed to obtain mutex lock");
    response_with_model(&repo.users)
}

// fn post_users(context: &Context) -> ServerFuture {
//     let repo = context.repo.lock().expect("Failed to obtain mutex lock");
//     let user = serde_json::from_str(context.body)?
//     response_with_model(&repo.users)
// }

fn not_found(_context: &Context) -> ServerFuture {
    let text = "Not found";
    Box::new(future::ok(
        Response::builder().status(404).body(text.into()).unwrap(),
    ))
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

// Reads body of request and response in Future format
fn read_body(body: hyper::Body) -> impl Future<Item = String, Error = Error> {
    body.fold(Vec::new(), |mut acc, chunk| {
        acc.extend_from_slice(&*chunk);
        future::ok::<_, hyper::Error>(acc)
    }).map_err(|e| e.context(ErrorKind::Hyper).into())
        .and_then(|bytes| {
            let bytes_clone = bytes.clone();
            String::from_utf8(bytes).map_err(|e| {
                e.context(format!("bytes: {:?}", &bytes_clone))
                    .context(ErrorKind::Utf8)
                    .into()
            })
        })
}

fn response_with_model<M>(model: &M) -> ServerFuture
where
    M: Debug + Serialize,
{
    Box::new(
        serde_json::to_string(&model)
            .map_err(|e| {
                e.context(format!("model: {:?}", &model))
                    .context(ErrorKind::Json)
                    .into()
            })
            .into_future()
            .map(|text| {
                Response::builder()
                    .status(200)
                    .header("Content-Type", "application/json")
                    .body(text.into())
                    .unwrap()
            }),
    )
}
