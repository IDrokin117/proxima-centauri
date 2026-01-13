pub enum ProxyRequests {
    Connect,
    ConnectWithoutAuth,
    ConnectInvalidAuth,
    Get,
    Malformed,
}

impl ProxyRequests {
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            // Valid CONNECT with correct auth
            // drokin_ii:o953zY7lnkYMEl5D -> Base64: ZHJva2luX2lpOm85NTN6WTdsbmtZTUVsNUQ=
            ProxyRequests::Connect => {
                b"CONNECT ident.me:443 HTTP/1.1\r\n\
                  Host: ident.me:443\r\n\
                  Proxy-Authorization: Basic ZHJva2luX2lpOm85NTN6WTdsbmtZTUVsNUQ=\r\n\
                  \r\n"
            }
            // CONNECT without Proxy-Authorization header
            ProxyRequests::ConnectWithoutAuth => {
                b"CONNECT example.com:443 HTTP/1.1\r\n\
                  Host: example.com:443\r\n\
                  \r\n"
            }
            // CONNECT with invalid credentials
            ProxyRequests::ConnectInvalidAuth => {
                b"CONNECT example.com:443 HTTP/1.1\r\n\
                  Host: example.com:443\r\n\
                  Proxy-Authorization: Basic aW52YWxpZDppbnZhbGlk\r\n\
                  \r\n"
            }
            // GET request (should return 405 Method Not Allowed)
            ProxyRequests::Get => {
                b"GET / HTTP/1.1\r\n\
                  Host: example.com\r\n\
                  \r\n"
            }
            // Malformed request
            ProxyRequests::Malformed => {
                b"INVALID REQUEST\r\n"
            }
        }
    }
}
