/// ## brief
/// HttpMethod enum
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

impl HttpMethod {
    /// ## brief
    /// Convert bytes to HttpMethod enum
    ///
    /// ## params:
    /// b - bytes
    ///
    /// ## return:
    /// HttpMethod
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        match b {
            b"GET" => Some(HttpMethod::GET),
            b"POST" => Some(HttpMethod::POST),
            b"PUT" => Some(HttpMethod::PUT),
            b"DELETE" => Some(HttpMethod::DELETE),
            b"PATCH" => Some(HttpMethod::PATCH),
            b"HEAD" => Some(HttpMethod::HEAD),
            b"OPTIONS" => Some(HttpMethod::OPTIONS),
            b"CONNECT" => Some(HttpMethod::CONNECT),
            b"TRACE" => Some(HttpMethod::TRACE),
            _ => None
        }
    }
}
