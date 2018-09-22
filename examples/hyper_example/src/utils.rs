use super::error::{Error, ErrorKind};
use super::types::ServerFuture;
use failure::Fail;
use futures::future;
use futures::prelude::*;
use hyper;
use hyper::Response;
use serde::Serialize;
use serde_json;
use std::fmt::Debug;

// Reads body of request and response in Future format
pub fn read_body(body: hyper::Body) -> impl Future<Item = String, Error = Error> {
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

pub fn response_with_model<M>(model: &M) -> ServerFuture
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
