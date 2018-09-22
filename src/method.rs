#[cfg(feature = "with_hyper")]
use hyper::Method as HyperMethod;

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
    TRACE,
}

#[cfg(feature = "with_hyper")]
impl From<HyperMethod> for Method {
    fn from(hm: HyperMethod) -> Method {
        match hm {
            HyperMethod::OPTIONS => Method::OPTIONS,
            HyperMethod::GET => Method::GET,
            HyperMethod::POST => Method::POST,
            HyperMethod::PUT => Method::PUT,
            HyperMethod::DELETE => Method::DELETE,
            HyperMethod::HEAD => Method::HEAD,
            HyperMethod::TRACE => Method::TRACE,
            HyperMethod::CONNECT => Method::CONNECT,
            HyperMethod::PATCH => Method::PATCH,
            _ => panic!("Not implemented hyper method in http_router lib"),
        }
    }
}
