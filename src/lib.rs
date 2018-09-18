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
    // (@test_pattern $request:expr, $method:expr, $path: expr,  $test_method: expr, /$($path_segment: ident)/+/:$id:ident => $hander:ident) => {{
    //     let mut s = String::new();
    //     $(
    //         s.push_str(stringify!($path_segment));
    //     )+
    //     println!("path: {}, id: {}", s, $id);
    //     Some(1)
    // }};

    (@one_route $request:expr, $method:expr, $path:expr, $expected_method:expr, /$($expected_path_segment:ident)/+ => $handler:ident) => {
        let mut s = String::new();
        $(
            s.push_str(stringify!($expected_path_segment));
        )+
        println!("Expected method: {}, path: {}", $method, s);
        None
    };

    (@many_routes $request:expr, $method:expr, $path:expr, $default:expr, $(GET $($rest:tt)+)*) => {
        let mut result = None;
        $(
            if result.is_none() {
                result = router!(@one_route $request, $method, $path, Method::GET, $($rest)+)
            }
        )*
        result.unwrap_or($default($request))
    };

    // Entry pattern
    (_ => $default:ident, $(matchers_tokens:tt)*) => {
        |request, method, path| => {
            router!(@many_routes request, method, path, $default, $(matchers_tokens)*)
        }
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    fn yo(x: u32) -> u32 {
        x
    }

    #[test]
    fn it_works() {
        // let x = 1;
        // let id = 2;
        // trace_macros!(true);
        // router!{1, 1, 1,
        //     GET (/users/:id => x),
        //     _ => yo
        // };
        // trace_macros!(false);
        // println!("{:?}", Method::POST);
        // assert_eq!(2 + 2, 4);

        trace_macros!(true);
        router!(
            _ => yo,
            GET /users/accounts/transactions => yo,
        )
    }
}
