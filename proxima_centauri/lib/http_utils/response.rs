pub enum ProxyResponse {
    ConnectionEstablished,
    Unauthorized,
    ProxyAuthRequired,
    MethodNotAllowed,
    TooManyRequests,
    QuotaExceeded,
}

impl ProxyResponse {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            ProxyResponse::ConnectionEstablished => b"HTTP/1.1 200 Connection Established\r\n\r\n",
            ProxyResponse::Unauthorized => b"HTTP/1.1 401 Unauthorized\r\n\r\n",
            ProxyResponse::ProxyAuthRequired => {
                b"HTTP/1.1 407 Proxy Authentication Required\r\n\r\n"
            }
            ProxyResponse::MethodNotAllowed => b"HTTP/1.1 405 Method Not Allowed\r\n\r\n",
            ProxyResponse::TooManyRequests =>  b"HTTP/1.1 429 Too Many Requests\r\n\r\n",
            ProxyResponse::QuotaExceeded =>  b"HTTP/1.1 403 Forbidden\r\n\r\n",
        }
    }
}
