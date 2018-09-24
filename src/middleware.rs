use hyper::rt::Future;
use hyper::{Request, Response, Body};
use hyper::service::Service;
use std::error::Error as StdError;
use super::Method;
use std::sync::Arc;

pub struct RouterMiddleware<T, C, E, F, R>
where F: Fn(Request<Body>) -> Box<Future<Item = C, Error = E>>,
R: Fn(C, Method, &str) -> Box<Future<Item = Response<Body>, Error = E>> + 'static,
C: 'static
{
    upstream: T,
    ctx_ctor: F,
    router: Arc<R>,
}

impl <T, C, E, F, R> RouterMiddleware<T, C, E, F, R>
where F: Fn(Request<Body>) -> Box<Future<Item = C, Error = E>>,
R: Fn(C, Method, &str) -> Box<Future<Item = Response<Body>, Error = E>>  + 'static,
C: 'static
{
    fn new(upstream: T, ctx_ctor: F, router: R) -> Self {
        RouterMiddleware {
            upstream,
            ctx_ctor,
            router: Arc::new(router),
        }
    }
}

impl <T, C, E, F, R> Service for RouterMiddleware<T, C, E, F, R>
where E: Into<Box<StdError + Send + Sync>>  + 'static,
R: Fn(C, Method, &str) -> Box<Future<Item = Response<Body>, Error = E>>  + 'static,
F: Fn(Request<Body>) -> Box<Future<Item = C, Error = E>>,
C: 'static,
T: Service,
T::Error: From<E>,
{

    type ReqBody = Body;
    type ResBody = Body;
    type Error = E;
    type Future = Box<Future<Item = Response<Body>, Error = E>>;

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let method: Method = req.method().clone().into();
        let path = req.uri().path().to_string();
        let router = self.router.clone();
        Box::new(
            (self.ctx_ctor)(req).and_then(move |ctx| {
                router(ctx, method, &path)
            })
        )
    }
}
