## Http router

This is a simple yet expressive router for http requests, abstract enough to be used with any http library on stable Rust.

light deps

### Getting started (for Hyper >= 0.12)

In your Cargo.toml

```toml
[dependencies]
http_router = "0.1"
```

In your lib.rs or main.rs:
```rust
#[macro_use]
extern crate http_router;
```

In your struct than implements Hyper `Service`:

```rust
// Each handler must have the same return type
// A good candidate might be a Box<Future<Item = hyper::Response, Error = Error>>
// The cost of this macro is next to zero, so it's ok to call it on each request
let router = router!(
    GET / => get_users,

    GET /users => get_users,
    POST /users => post_users,
    PUT /users/{user_id: usize} => put_users,
    DELETE /users/{user_id: usize} => delete_users,

    GET /users/{user_id: usize}/transactions => get_transactions,
    POST /users/{user_id: usize}/transactions => post_transactions,
    PUT /users/{user_id: usize}/transactions/{hash: String} => put_transactions,
    DELETE /users/{user_id: usize}/transactions/{hash: String} => delete_transactions,

    _ => not_found,
);

let path = req.uri.path();
let ctx = Context { ... };
// This will return a value of the matched handler's return type
// E.g. the aforementioned Box<Future<Item = hyper::Response, Error = Error>>
router(ctx, req.method.into(), path)
```

A file with handlers implementation

```rust
// Params from a route become handlers' typed params.
// If a param's type doesn't match (e.g. you supplied `sdf` as a user id, that must be `usize`)
// then this route counts as non-matching

type ServerFuture = Box<Future<Item = hyper::Response, Error = Error>>;

pub fn get_users(context: &Context) -> ServerFuture {
    ...
}

pub fn post_users(context: &Context) -> ServerFuture {
    ...
}

pub fn put_users(context: &Context, user_id: usize) -> ServerFuture {
    ...
}

pub fn delete_users(context: &Context, id: usize) -> ServerFuture {
    ...
}

pub fn get_transactions(context: &Context, user_id: usize) -> ServerFuture {
    ...
}

pub fn post_transactions(context: &Context, user_id: usize) -> ServerFuture {
    ...
}

pub fn put_transactions(context: &Context, user_id: usize, hash: String) -> ServerFuture {
    ...
}

pub fn delete_transactions(context: &Context, user_id: usize, hash: String) -> ServerFuture {
    ...
}

pub fn not_found(_context: &Context) -> ServerFuture {
    ...
}

```

See [examples folder](examples/hyper_example) for a complete Hyper example

### Using with other http libs

By default this crate is configured to be used with `hyper >=0.12`. If you want to use it with other libs, you might want to opt out of default features for this crate. So in your Cargo.toml:

```toml
[dependencies]
http_router = config = { version = "0.1", default-features = false}
```

The `router!` macro is independent of any framework. However, it returns a closure that takes 3 params - `context`, `method` and `path`. You need to supply these 3 params from your http lib.

`context` is a param of your user-defined type. e.g. `Context`. It will be passed as a first argument to all of your handlers. You can put there any values like database interfaces and http clients as you like.

`method` is a param of type Method defined in `http_router` lib. It is one of `GET`, `POST`, etc.

`path` is a `&str` which is the current route for a request.

Once you define these 3 params, you can use the `router!` macro for routing.

_NOTE: By default this crate is configured to be used with hyper >=0.12. If you want to use it with other libs, you need to opt out of default features for this crate_

### Benchmarks

Right now the router with 10 routes takes approx 50 microseconds for one match
