/// ## brief
/// HttpVersion enum
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20 
}

impl HttpVersion {
    /// ## brief
    /// Convert bytes to HttpVersion enum
    ///
    /// ## params:
    /// b - bytes
    ///
    /// ## return:
    /// HttpVersion
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        match b {
            b"HTTP/1.0" => Some(HttpVersion::Http10),
            b"HTTP/1.1" => Some(HttpVersion::Http11),
            b"HTTP/2.0" => Some(HttpVersion::Http20),
            _ => None
        }
    }

    /// ## brief
    /// Convert version it self as str
    ///
    /// ## params:
    /// &self
    ///
    /// ## return:
    /// &'static str
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http20 => "HTTP/2.0",
        }
    }
}
