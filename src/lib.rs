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
    (@parse_type $value:ident, $ty:ty) => { $value.parse::<$ty>().unwrap() };

    (@call_pure $request:expr, $handler:ident, $params:expr, $({$id:ident : $ty:ty : $idx:expr})*) => {{
        $handler($request, $({
            let value = $params[$idx];
            router!(@parse_type value, $ty)
        }),*)
    }};

    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident : $ty1:ty} $($p1:ident)*) => {{
        // println!(stringify!($ty1));
        // $handler($request, $params[0].parse::<$ty1>().unwrap())
        router!(@call_pure $request, $handler, $params, {$id1 : $ty1 : 0})
    }};

    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+ {$id1:ident} $($p1:ident)*) => {{
        $handler($request, $params[0])
    }};
    (@call, $request:expr, $handler:ident, $params:expr, $($p:ident)+) => {{
        $handler($request)
    }};
    (@one_route_with_method $request:expr, $method:expr, $path:expr, $default:expr, $expected_method: expr, $handler:ident, $($path_segment:tt)*) => {{
        let mut s = String::new();
        $(
            s.push('/');
            let path_segment = stringify!($path_segment);
            if path_segment.starts_with('{') {
                s.push_str(r#"([\w-]+)"#);
            } else {
                s.push_str(stringify!($path_segment));
            }
        )+
        let re = regex::Regex::new(&s).unwrap();
        if let Some(captures) = re.captures($path) {
            let matches: Vec<&str> = captures.iter().skip(1).filter(|x| x.is_some()).map(|x| x.unwrap().as_str()).collect();
            println!("Matched: {:?}, path: {}, regex: {}, matches: {:?}", $method, $path, s, matches);
            Some(router!(@call, $request, $handler, matches, $($path_segment)*))
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

    fn yo1(x: u32, y: &str) -> u32 {
        println!("Called yo1 with {} and {}", x, y);
        x
    }

    fn yoint(x: u32, y: u32) -> u32 {
        println!("Called yo1 with {} and {}", x, y);
        x
    }

    fn yostr(x: u32, y: String) -> u32 {
        println!("Called yo1 with {} and {}", x, y);
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
            GET /users/transactions/{transaction_id: String}/accounts => yostr
            // GET /users/transactions/{transaction_id}/accounts => yo1
            // GET /users/transactions => yo
        );

        trace_macros!(false);
        // router(32, Method::GET, "/users/123/accounts/sdf/transactions/123");
        router(32, Method::GET, "/users/transactions/12/accounts");
    }
}
