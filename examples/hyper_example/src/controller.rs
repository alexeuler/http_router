use super::error::ErrorKind;
use super::repo::{Repo, Transaction, User};
use super::types::ServerFuture;
use super::utils::response_with_model;
use failure::Fail;
use futures::prelude::*;
use hyper::Response;
use serde_json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Context {
    pub repo: Arc<Mutex<Repo>>,
    pub body: String,
}

pub fn get_users(context: &Context) -> ServerFuture {
    let repo = context.repo.lock().expect("Failed to obtain mutex lock");
    response_with_model(&repo.users)
}

pub fn post_users(context: &Context) -> ServerFuture {
    let repo_arc_mutex = context.repo.clone();
    let context_clone = context.clone();
    Box::new(
        serde_json::from_str(&context.body)
            .into_future()
            .map_err(move |e| {
                e.context(format!("body: {}", &context_clone.body))
                    .context(ErrorKind::Json)
                    .into()
            })
            .and_then(move |user: User| {
                let mut repo = repo_arc_mutex.lock().expect("Failed to obtain mutex lock");
                repo.users.push(user.clone());
                response_with_model(&user)
            }),
    )
}

pub fn not_found(_context: &Context) -> ServerFuture {
    let text = "Not found";
    Box::new(Ok(Response::builder().status(404).body(text.into()).unwrap()).into_future())
}
