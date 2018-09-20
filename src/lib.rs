#![allow(unused_mut)]

//! This is an abstract http router that can be used with any library, incl. Hyper, Actix, etc.
//! Usage:
//!
//! ```
//! let router = router!(request,
//!     GET /users/:user_id/widgets => users_widgets_list,
//!     POST /users/:user_id/widgets => users_widgets_create,
//! );
//!
//! router(request)
//! 
//! Working types: anything that implements FromStr. Since &str doesn't implement FromStr - use String instead.
//!
//! fn users_widgets_list(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//!
//! fn users_widgets_create(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//! ```

extern crate regex;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

lazy_static! {
    static ref REGEXES: Arc<Mutex<RefCell<HashMap<String, regex::Regex>>>> = {
        Arc::new(Mutex::new(RefCell::new(HashMap::new())))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE
}

pub fn create_regex(s: &str) -> regex::Regex {
    let mut result: Option<regex::Regex> = None;
    {
        let regexes_cell = REGEXES.lock().expect("Failed to obtain mutex lock");
        let regexes = regexes_cell.borrow();
        result = regexes.get(s).cloned();
    };
    result.unwrap_or_else(|| {
        let re = regex::Regex::new(s).unwrap();
        let regexes_cell = REGEXES.lock().expect("Failed to obtain mutex lock");
        let mut regexes = regexes_cell.borrow_mut();
        regexes.insert(s.to_string(), re.clone());
        re
    })
}

#[macro_export]
macro_rules! router {
    // convert params from string
    (@parse_type $value:expr, $ty:ty) => {{
        let maybe_val = $value.parse::<$ty>();
        if maybe_val.is_err() { return None };
        maybe_val.unwrap()
    }};

    // call handler with params
    (@call_pure $context:expr, $request:expr, $handler:ident, $params:expr, $({$id:ident : $ty:ty : $idx:expr}),*) => {{
        $handler($context, $request, $({
            let value = $params[$idx];
            router!(@parse_type value, $ty)
        }),*)
    }};

    // Extract params from route, 0 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)*) => {{
        $handler($context, $request)
    }};

    // Extract params from route, 1 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0})
    }};

    // Extract params from route, 2 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1})
    }};

    // Extract params from route, 3 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2})
    }};

    // Extract params from route, 4 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3})
    }};

    // Extract params from route, 5 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4})
    }};

    // Extract params from route, 6 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)* {$id6:ident : $ty6:ty} $($p6:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4}, {$id6 : $ty6 : 5})
    }};

    // Extract params from route, 7 params case
    (@call, $context:expr, $request:expr, $handler:ident, $params:expr, $($p:ident)* {$id1:ident : $ty1:ty} $($p1:ident)* {$id2:ident : $ty2:ty} $($p2:ident)* {$id3:ident : $ty3:ty} $($p3:ident)* {$id4:ident : $ty4:ty} $($p4:ident)* {$id5:ident : $ty5:ty} $($p5:ident)* {$id6:ident : $ty6:ty} $($p6:ident)* {$id7:ident : $ty7:ty} $($p7:ident)*) => {{
        router!(@call_pure $context, $request, $handler, $params, {$id1 : $ty1 : 0}, {$id2 : $ty2 : 1}, {$id3 : $ty3 : 2}, {$id4 : $ty4 : 3}, {$id5 : $ty5 : 4}, {$id6 : $ty6 : 5}, {$id6 : $ty6 : 6})
    }};

    // Test a particular route for match and forward to @call if there is match
    (@one_route_with_method $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, $expected_method: expr, $handler:ident, $($path_segment:tt)*) => {{
        if $method != $expected_method { return None };
        let mut s = "^".to_string();
        $(
            s.push('/');
            let path_segment = stringify!($path_segment);
            if path_segment.starts_with('{') {
                s.push_str(r#"([\w-]+)"#);
            } else {
                s.push_str(path_segment);
            }
        )*
        // handle home case
        if s.len() == 1 { s.push('/') }
        s.push('$');
        let re = $crate::create_regex(&s);
        if let Some(captures) = re.captures($path) {
            let _matches: Vec<&str> = captures.iter().skip(1).filter(|x| x.is_some()).map(|x| x.unwrap().as_str()).collect();
            Some(router!(@call, $context, $request, $handler, _matches, $($path_segment)*))
        } else {
            None
        }
    }};

    // Transform GET token to Method::GET
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, GET, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::GET, $handler, $($path_segment)*)
    };

    // Transform POST token to Method::POST
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, POST, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::POST, $handler, $($path_segment)*)
    };
    // Transform PUT token to Method::PUT
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, PUT, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::PUT, $handler, $($path_segment)*)
    };
    // Transform PATCH token to Method::PATCH
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, PATCH, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::PATCH, $handler, $($path_segment)*)
    };
    // Transform DELETE token to Method::DELETE
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, DELETE, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::DELETE, $handler, $($path_segment)*)
    };
    // Transform OPTIONS token to Method::OPTIONS
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, OPTIONS, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::OPTIONS, $handler, $($path_segment)*)
    };

    // Transform HEAD token to Method::HEAD
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, HEAD, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::HEAD, $handler, $($path_segment)*)
    };

    // Transform TRACE token to Method::TRACE
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, TRACE, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::TRACE, $handler, $($path_segment)*)
    };

    // Transform CONNECT token to Method::CONNECT
    (@one_route $context:expr, $request:expr, $method:expr, $path:expr, $default:expr, CONNECT, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $context, $request, $method, $path, $default, Method::CONNECT, $handler, $($path_segment)*)
    };

    // Entry pattern
    ($($method_token:ident $(/$path_segment:tt)+ => $handler:ident,)* _ => $default:ident $(,)*) => {{
        move |context, request, method: Method, path: String| {
            let mut result = None;
            $(
                if result.is_none() {
                    // we use closure here so that we could make early return from macros inside of it
                    let closure = || {
                        router!(@one_route context, request, method, &path, $default, $method_token, $handler, $($path_segment)*)
                    };
                    result = closure();
                }
            )*
            result.unwrap_or_else(|| $default(context, request))
        }
    }};

    // Entry pattern - with home first
    ($home_method_token:ident / => $home_handler:ident, $($method_token:ident $(/$path_segment:tt)+ => $handler:ident,)* _ => $default:ident $(,)*) => {{
        move |context, request, method: Method, path: String| {
            let closure = || {
                router!(@one_route context, request, method, &path, $default, $home_method_token, $home_handler,)
            };
            let mut result = closure();
            $(
                if result.is_none() {
                    // we use closure here so that we could make early return from macros inside of it
                    let closure = || {
                        router!(@one_route context, request, method, &path, $default, $method_token, $handler, $($path_segment)*)
                    };
                    result = closure();
                }
            )*
            result.unwrap_or_else(|| $default(context, request))
        }
    }};

    // Entry pattern - default only
    (_ => $default:ident $(,)*) => {
        move |context, request, _method: Method, _path: String| {
            $default(context, request)
        }
    }
}

#[cfg(test)]
mod tests {
    // extern crate test;    
    extern crate rand;

    // use self::test::Bencher;
    use std::thread;
    use super::*;

    const NUMBER_OF_THREADS_FOR_REAL_LIFE_TEST: usize = 4;
    const NUMBER_OF_TESTS_FOR_REAL_LIFE_TEST: usize = 3000;

    #[test]
    fn test_real_life() {
        let get_users = |_: (), _: ()| "get_users".to_string();
        let post_users = |_: (), _: ()| "post_users".to_string();
        let patch_users = |_: (), _: (), id: u32| format!("patch_users({})", id);
        let delete_users = |_: (), _: (), id: u32| format!("delete_users({})", id);
        let get_transactions = |_: (), _: (), id: u32| format!("get_transactions({})", id);
        let post_transactions = |_: (), _: (), id: u32| format!("post_transactions({})", id);
        let patch_transactions = |_: (), _: (), id: u32, hash: String| format!("patch_transactions({}, {})", id, hash);
        let delete_transactions = |_: (), _: (), id: u32, hash: String| format!("delete_transactions({}, {})", id, hash);
        let fallback = |_: (), _: ()| "404".to_string();

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
            (Method::GET, "/users/534/transactions", "get_transactions(534)"),
            (Method::POST, "/users/534/transactions", "post_transactions(534)"),
            (Method::PATCH, "/users/534/transactions/0x234", "patch_transactions(534, 0x234)"),
            (Method::DELETE, "/users/534/transactions/0x234", "delete_transactions(534, 0x234)"),
            (Method::DELETE, "/users/5d34/transactions/0x234", "404"),
            (Method::POST, "/users/534/transactions/0x234", "404"),
            (Method::GET, "/u", "404"),
            (Method::POST, "/", "404"),
        ];
        for test_case in test_cases.into_iter() {
            let (method, path, expected) = test_case.clone();
            assert_eq!(router((), (), method.clone(), path.to_string()), expected.to_string());
        }

        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
        for _ in 0..NUMBER_OF_THREADS_FOR_REAL_LIFE_TEST {
            let handle = thread::spawn(move || {
                for _ in 0..NUMBER_OF_TESTS_FOR_REAL_LIFE_TEST {
                    let number = rand::random::<usize>() % test_cases.len();
                    let test_case = test_cases[number];
                    let (method, path, expected) = test_case;
                    assert_eq!(router((), (), method.clone(), path.to_string()), expected.to_string());
                }
            });
            threads.push(handle);
        }
        for thread in threads {
            let _ = thread.join();
        }
    }

    #[test]
    fn test_home() {
        let get_home = |_: (), _: ()| "get_home";
        let unreachable = |_: (), _: ()| unreachable!();
        let router = router!(
            GET / => get_home,
            _ => unreachable
        );
        assert_eq!(router((), (), Method::GET, "/".to_string()), "get_home");
    }

    #[test]
    fn test_fallback() {
        let home = |_: (), _: ()| "home";
        let users = |_: (), _: ()| "users";
        let fallback = |_: (), _: ()| "fallback";
        let router = router!(
            GET / => home,
            POST /users => users,
            _ => fallback
        );
        assert_eq!(router((), (), Method::GET, "/".to_string()), "home");
        assert_eq!(router((), (), Method::POST, "/users".to_string()), "users");
        assert_eq!(router((), (), Method::GET, "/users".to_string()), "fallback");
        assert_eq!(router((), (), Method::GET, "/us".to_string()), "fallback");
        assert_eq!(router((), (), Method::PATCH, "/".to_string()), "fallback");
    }

    #[test]
    fn test_verbs() {
        let get_test = |_: (), _: ()| Method::GET;
        let post_test = |_: (), _: ()| Method::POST;
        let put_test = |_: (), _: ()| Method::PUT;
        let patch_test = |_: (), _: ()| Method::PATCH;
        let delete_test = |_: (), _: ()| Method::DELETE;
        let connect_test = |_: (), _: ()| Method::CONNECT;
        let options_test = |_: (), _: ()| Method::OPTIONS;
        let trace_test = |_: (), _: ()| Method::TRACE;
        let head_test = |_: (), _: ()| Method::HEAD;
        let panic_test = |_: (), _: ()| unreachable!();
        let router = router!(
            GET /users => get_test,
            POST /users => post_test,
            PUT /users => put_test,
            PATCH /users => patch_test,
            DELETE /users => delete_test,
            OPTIONS /users => options_test,
            CONNECT /users => connect_test,
            TRACE /users => trace_test,
            HEAD /users => head_test,
            _ => panic_test
        );

        assert_eq!(router((), (), Method::GET, "/users".to_string()), Method::GET);
        assert_eq!(router((), (), Method::POST, "/users".to_string()), Method::POST);
        assert_eq!(router((), (), Method::PUT, "/users".to_string()), Method::PUT);
        assert_eq!(router((), (), Method::PATCH, "/users".to_string()), Method::PATCH);
        assert_eq!(router((), (), Method::DELETE, "/users".to_string()), Method::DELETE);
        assert_eq!(router((), (), Method::OPTIONS, "/users".to_string()), Method::OPTIONS);
        assert_eq!(router((), (), Method::TRACE, "/users".to_string()), Method::TRACE);
        assert_eq!(router((), (), Method::CONNECT, "/users".to_string()), Method::CONNECT);
        assert_eq!(router((), (), Method::HEAD, "/users".to_string()), Method::HEAD);
    }

    #[test]
    fn test_params_number() {
        let zero = |_: (), _: ()| String::new();
        let one = |_: (), _: (), p1: String| format!("{}", &p1);
        let two = |_: (), _: (), p1: String, p2: String| format!("{}{}", &p1, &p2);
        let three = |_: (), _: (), p1: String, p2: String, p3: String| format!("{}{}{}", &p1, &p2, &p3);
        let four = |_: (), _: (), p1: String, p2: String, p3: String, p4: String| format!("{}{}{}{}", &p1, &p2, &p3, &p4);
        let five = |_: (), _: (), p1: String, p2: String, p3: String, p4: String, p5: String| format!("{}{}{}{}{}", &p1, &p2, &p3, &p4, &p5);
        let six = |_: (), _: (), p1: String, p2: String, p3: String, p4: String, p5: String, p6: String| format!("{}{}{}{}{}{}", &p1, &p2, &p3, &p4, &p5, &p6);
        let seven = |_: (), _: (), p1: String, p2: String, p3: String, p4: String, p5: String, p6: String, p7: String| format!("{}{}{}{}{}{}{}", &p1, &p2, &p3, &p4, &p5, &p6, &p7);
        let unreachable = |_: (), _: ()| unreachable!();
        let router = router!(
            GET /users => zero,
            GET /users/{p1: String} => one,
            GET /users/{p1: String}/users2/{p2: String} => two,
            GET /users/{p1: String}/users2/{p2: String}/users3/{p3: String} => three,
            GET /users/{p1: String}/users2/{p2: String}/users3/{p3: String}/users4/{p4: String} => four,
            GET /users/{p1: String}/users2/{p2: String}/users3/{p3: String}/users4/{p4: String}/users5/{p5: String} => five,
            GET /users/{p1: String}/users2/{p2: String}/users3/{p3: String}/users4/{p4: String}/users5/{p5: String}/users6/{p6: String} => six,
            GET /users/{p1: String}/users2/{p2: String}/users3/{p3: String}/users4/{p4: String}/users5/{p5: String}/users6/{p6: String}/users7/{p7: String} => seven,
            _ => unreachable,
        );

        assert_eq!(router((), (), Method::GET, "/users".to_string()), "");
        assert_eq!(router((), (), Method::GET, "/users/id1".to_string()), "id1");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2".to_string()), "id1id2");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2/users3/id3".to_string()), "id1id2id3");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2/users3/id3/users4/id4".to_string()), "id1id2id3id4");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2/users3/id3/users4/id4/users5/id5".to_string()), "id1id2id3id4id5");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2/users3/id3/users4/id4/users5/id5/users6/id6".to_string()), "id1id2id3id4id5id6");
        assert_eq!(router((), (), Method::GET, "/users/id1/users2/id2/users3/id3/users4/id4/users5/id5/users6/id6/users7/id7".to_string()), "id1id2id3id4id5id6id7");
    }
}


// cargo +nightly rustc -- -Zunstable-options --pretty=expanded
