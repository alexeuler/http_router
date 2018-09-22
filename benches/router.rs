#![feature(test)]

extern crate http_router;
extern crate rand;
extern crate regex;
extern crate test;

use test::Bencher;
#[macro_use]
use http_router::*;

#[bench]
fn bench_router(b: &mut Bencher) {
    let get_users = |_: ()| "get_users".to_string();
    let post_users = |_: ()| "post_users".to_string();
    let patch_users = |_: (), id: u32| format!("patch_users({})", id);
    let delete_users = |_: (), id: u32| format!("delete_users({})", id);
    let get_transactions = |_: (), id: u32| format!("get_transactions({})", id);
    let post_transactions = |_: (), id: u32| format!("post_transactions({})", id);
    let patch_transactions =
        |_: (), id: u32, hash: String| format!("patch_transactions({}, {})", id, hash);
    let delete_transactions =
        |_: (), id: u32, hash: String| format!("delete_transactions({}, {})", id, hash);
    let fallback = |_: ()| "404".to_string();

    let router = router!(
        GET / => get_users,
        GET /users => get_users,
        POST /users => post_users,
        PATCH /users/{user_id: u32} => patch_users,
        DELETE /users/{user_id: u32} => delete_users,
        GET /users/{user_id: u32}/transactions => get_transactions,
        POST /users/{user_id: u32}/transactions => post_transactions,
        PATCH /users/{user_id: u32}/transactions/{hash: String} => patch_transactions,
        DELETE /users/{user_id: u32}/transactions/{hash: String} => delete_transactions,
        _ => fallback,
    );
    let test_cases = [
        (Method::GET, "/", "get_users"),
        (Method::GET, "/users", "get_users"),
        (Method::POST, "/users", "post_users"),
        (Method::PATCH, "/users/12", "patch_users(12)"),
        (Method::DELETE, "/users/132134", "delete_users(132134)"),
        (
            Method::GET,
            "/users/534/transactions",
            "get_transactions(534)",
        ),
        (
            Method::POST,
            "/users/534/transactions",
            "post_transactions(534)",
        ),
        (
            Method::PATCH,
            "/users/534/transactions/0x234",
            "patch_transactions(534, 0x234)",
        ),
        (
            Method::DELETE,
            "/users/534/transactions/0x234",
            "delete_transactions(534, 0x234)",
        ),
        (Method::DELETE, "/users/5d34/transactions/0x234", "404"),
        (Method::POST, "/users/534/transactions/0x234", "404"),
        (Method::GET, "/u", "404"),
        (Method::POST, "/", "404"),
    ];

    b.iter(|| {
        let number = rand::random::<usize>() % test_cases.len();
        let (method, path, expected) = test_cases[number];
        let _ = router((), method.clone(), path);
    });
}

#[bench]
fn bench_plain_regex(b: &mut Bencher) {
    let re = regex::Regex::new(r#"/users/([\w-]+)/transactions/([\w-]+)"#).unwrap();
    b.iter(|| {
        // number of routes in router
        for i in 0..9 {
            for matches in re.captures("/users/234/transactions/dfgd") {}
        }
    });
}
