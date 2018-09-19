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
//! fn users_widgets_list(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//!
//! fn users_widgets_create(request, user_id: u32) -> impl Future<Item = (), Error = ()> {
//!     unimplemented!()
//! }
//! ```

#![feature(trace_macros)]
#[allow(unused_macros)]

extern crate regex;


#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

macro_rules! router {
    (@call 1, $request:expr, $handler:ident, $($rest:tt)+) => {{
        panic!("Unexpected call");
    }};

    (@call 0, $request:expr, $handler:ident, $path_segment:tt) => {{
        let path_segment = stringify!($path_segment);
        if (path_segment.starts_with('{')) {
            router!(@call 1, $request, $handler, $path_segment)
        } else {
            $handler($request)
        }
    }};

    (@call 0, $request:expr, $handler:ident, $path_segment:tt $($rest:tt)+) => {{
        let path_segment = stringify!($path_segment);
        if (path_segment.starts_with('{')) {
            router!(@call 1, $request, $handler, $path_segment $($rest)+)
        } else {
            router!(@call 0, $request, $handler, $($rest)+)
        }
    }};

    (@one_route_with_method $request:expr, $method:expr, $path:expr, $default:expr, $expected_method: expr, $handler:ident, $($path_segment:tt)*) => {{
        let mut s = String::new();
        $(
            s.push_str(r#"/"#);
            let path_segment = stringify!($path_segment);
            if (path_segment.starts_with('{')) {
                s.push_str(r#"([\w-]+)"#);
            } else {
                s.push_str(stringify!($path_segment));
            }
        )+
        let re = regex::Regex::new(&s).unwrap();
        if re.is_match($path) {
            println!("Matched: {:?}, path: {}, regex: {}", $method, $path, s);
            Some(router!(@call 0, $request, $handler, $($path_segment)*))
        } else {
            println!("Didn't match: {:?}, path: {}, regex: {}", $method, $path, s);
            None
        }
    }};

    (@one_route $request:expr, $method:expr, $path:expr, $default:expr, GET, $handler:ident, $($path_segment:tt)*) => {
        router!(@one_route_with_method $request, $method, $path, $default, Method::GET, $handler, $($path_segment)*)
    };

    (@many_routes $request:expr, $method:expr, $path:expr, $default:expr, $($method_token:tt $(/$path_segment:tt)* => $handler:ident),*) => {{
        let mut result = None;
        $(
            if result.is_none() {
                result = router!(@one_route $request, $method, $path, $default, $method_token, $handler, $($path_segment)*)
            }
        )*
        result.unwrap_or_else(|| $default($request))
    }};

    // Entry pattern
    (_ => $default:ident, $($matchers_tokens:tt)*) => {
        |request, method: Method, path: &str| {
            router!(@many_routes request, method, path, $default, $($matchers_tokens)*)
        }
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    fn yo(x: u32) -> u32 {
        println!("Called yo with {}", x);
        x
    }

    #[test]
    fn it_works() {
        trace_macros!(true);
        // let router = router!(
        //     _ => yo,
        //     GET /users/{user_id}/accounts/{account_id}/transactions/{transaction_id} => yo
        // );
        let router = router!(
            _ => yo,
            GET /users/transactions => yo
        );

        trace_macros!(false);
        // router(32, Method::GET, "/users/123/accounts/sdf/transactions/123");
        router(32, Method::GET, "/users/transactions");
    }
}
