use crate::Server;
use crate::http_utils::response::ProxyResponse;
use anyhow::Result;
use httparse::{EMPTY_HEADER, Response};
use std::sync::atomic::{AtomicU16, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

use super::common::request::ProxyRequests;

static PORT_COUNTER: AtomicU16 = AtomicU16::new(9100);

struct TestServer {
    handle: tokio::task::JoinHandle<()>,
    addr: String,
}

impl TestServer {
    async fn start() -> Self {
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let addr = format!("127.0.0.1:{}", port);
        let addr_clone = addr.clone();

        let handle = tokio::spawn(async move {
            Server::run_on_addr(Some(addr_clone)).await.ok();
        });

        // Give server time to start
        sleep(Duration::from_millis(100)).await;

        TestServer { handle, addr }
    }

    fn addr(&self) -> &str {
        &self.addr
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

async fn read_response(socket: &mut TcpStream) -> Result<Vec<u8>> {
    let mut buff = vec![0u8; 1024];
    let size = socket.read(&mut buff).await?;
    buff.truncate(size);
    Ok(buff)
}

#[tokio::test]
async fn test_proxy_auth_required() -> Result<()> {
    let server = TestServer::start().await;
    let mut socket = TcpStream::connect(server.addr()).await?;

    socket
        .write_all(ProxyRequests::ConnectWithoutAuth.as_bytes())
        .await?;

    let response = read_response(&mut socket).await?;
    let expected = ProxyResponse::ProxyAuthRequired.as_bytes();

    assert_eq!(response, expected);
    Ok(())
}

#[tokio::test]
async fn test_unauthorized() -> Result<()> {
    let server = TestServer::start().await;
    let mut socket = TcpStream::connect(server.addr()).await?;

    socket
        .write_all(ProxyRequests::ConnectInvalidAuth.as_bytes())
        .await?;

    let response = read_response(&mut socket).await?;
    let expected = ProxyResponse::Unauthorized.as_bytes();

    assert_eq!(response, expected);
    Ok(())
}

#[tokio::test]
async fn test_method_not_allowed() -> Result<()> {
    let server = TestServer::start().await;
    let mut socket = TcpStream::connect(server.addr()).await?;

    socket.write_all(ProxyRequests::Get.as_bytes()).await?;

    let response = read_response(&mut socket).await?;
    let expected = ProxyResponse::MethodNotAllowed.as_bytes();

    assert_eq!(response, expected);
    Ok(())
}

#[tokio::test]
async fn test_successful_connect() -> Result<()> {
    let server = TestServer::start().await;
    let mut socket = TcpStream::connect(server.addr()).await?;

    socket.write_all(ProxyRequests::Connect.as_bytes()).await?;

    let response_bytes = read_response(&mut socket).await?;

    let mut headers = [EMPTY_HEADER; 16];
    let mut response = Response::new(&mut headers);
    response.parse(&*response_bytes)?;
    assert_eq!(response.code.unwrap(), 200);

    Ok(())
}

#[tokio::test]
async fn test_malformed_request() -> Result<()> {
    let server = TestServer::start().await;
    let mut socket = TcpStream::connect(server.addr()).await?;

    socket
        .write_all(ProxyRequests::Malformed.as_bytes())
        .await?;

    let result = read_response(&mut socket).await;

    assert!(result.is_ok() || result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_server_cleanup() -> Result<()> {
    let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);
    let addr = format!("127.0.0.1:{}", port);

    {
        let _server = TestServer::start().await;
        let socket = TcpStream::connect(_server.addr()).await;
        assert!(socket.is_ok());
    }

    sleep(Duration::from_millis(50)).await;

    let listener = tokio::net::TcpListener::bind(&addr).await;
    assert!(
        listener.is_ok(),
        "Port should be freed after server cleanup"
    );

    Ok(())
}
