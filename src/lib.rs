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

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

#[allow(unused_macros)]

macro_rules! router {
    // Entry pattern
    ($request:expr, $method:expr, $path: expr, $(GET /$($path_segment: ident)/+/:$id:ident => $handler: ident),*, _ => $default:ident) => {
        let mut result = None;
        $(
            if result.is_none() {
                result = router!(@test_pattern $request, $method, $path, Method::GET, $($path_segment)+, $id, $handler)
            }
        )*
        result.unwrap_or($default($request))
    };

    (@test_pattern $request:expr, $method:expr, $path: expr, $test_method: expr, $($path_segment: ident)+, $id:ident, $handler:ident) => {{
        let mut s = String::new();
        $(
            s.push_str(stringify!($path_segment));
        )+
        println!("path: {}, id: {}", s, $id);
        Some(1)
    }};


    (GET $($tail:tt)*) => {
        router!(Method::GET, $($tail)*)
    };
    (POST $($tail:tt)*) => {
        router!(Method::POST, $($tail)*)
    };
    ($route:expr, $x: tt) => {
        println!("{:?}", $route);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yo(x: u32) -> u32 {
        x
    }

    #[test]
    fn it_works() {
        let x = 1;
        let id = 2;
        router!{1, 1, 1,
            GET /users/:id => x,
            _ => yo
        };
        // println!("{:?}", Method::POST);
        assert_eq!(2 + 2, 4);
    }
}
