use super::error::ErrorKind;
use super::repo::{Repo, Transaction, User};
use super::types::ServerFuture;
use super::utils::response_with_model;
use failure::Fail;
use futures::prelude::*;
use hyper::{Body, Response};
use serde_json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Context {
    pub repo: Arc<Mutex<Repo>>,
    pub body: String,
}

pub fn get_users(context: &Context) -> ServerFuture {
    let repo = context.repo.lock().expect("Failed to obtain mutex lock");
    response_with_model(&repo.get_users())
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
                let user = repo.create_user(user);
                response_with_model(&user)
            }),
    )
}

pub fn put_users(context: &Context, _user_id: usize) -> ServerFuture {
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
                let user = repo.update_user(user);
                response_with_model(&user)
            }),
    )
}

pub fn delete_users(context: &Context, id: usize) -> ServerFuture {
    let mut repo = context.repo.lock().expect("Failed to obtain mutex lock");
    repo.delete_user(id);
    Box::new(Ok(Response::builder().status(204).body(Body::empty()).unwrap()).into_future())
}

pub fn get_transactions(context: &Context, user_id: usize) -> ServerFuture {
    let mut repo = context.repo.lock().expect("Failed to obtain mutex lock");
    Box::new(
        repo.get_transactions(user_id)
            .into_future()
            .and_then(|txs| response_with_model(&txs)),
    )
}

pub fn post_transactions(context: &Context, user_id: usize) -> ServerFuture {
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
            .and_then(move |tx: Transaction| {
                let mut repo = repo_arc_mutex.lock().expect("Failed to obtain mutex lock");
                repo.create_transaction(user_id, tx)
            })
            .and_then(|tx| response_with_model(&tx)),
    )
}

pub fn put_transactions(context: &Context, user_id: usize, _hash: String) -> ServerFuture {
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
            .and_then(move |tx: Transaction| {
                let mut repo = repo_arc_mutex.lock().expect("Failed to obtain mutex lock");
                repo.update_transaction(user_id, tx)
            })
            .and_then(|tx| response_with_model(&tx)),
    )
}

pub fn delete_transactions(context: &Context, user_id: usize, hash: String) -> ServerFuture {
    let mut repo = context.repo.lock().expect("Failed to obtain mutex lock");
    Box::new(
        repo.delete_transaction(user_id, hash)
            .map(|_| Response::builder().status(204).body(Body::empty()).unwrap())
            .into_future(),
    )
}

pub fn not_found(_context: &Context) -> ServerFuture {
    let text = "Not found";
    Box::new(Ok(Response::builder().status(404).body(text.into()).unwrap()).into_future())
}
