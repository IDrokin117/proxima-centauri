mod database;
mod statistics;

use crate::database::Database;
use crate::statistics::Statistics;
use anyhow::{anyhow, bail, Result};
use base64::{engine::general_purpose, Engine as _};
use httparse::{Request, EMPTY_HEADER};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{copy_bidirectional, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, span, trace, Level};
use tracing_subscriber;

struct Config {
    port: String,
    host: String,
    connection_timeout: u64,
}
impl Config {
    fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn init() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
}

fn build_config() -> Config {
    Config {
        port: dotenv::var("PROXY_PORT").unwrap_or(String::from("9090")),
        host: dotenv::var("PROXY_HOST").unwrap_or(String::from("127.0.0.1")),
        connection_timeout: 60,
    }
}
pub async fn run() -> Result<()> {
    init();
    let config = Arc::new(build_config());
    let database = Arc::new(Database::new_persistence());
    let statistics = Arc::new(Mutex::new(Statistics::new()));
    let global_span = span!(Level::TRACE, "global-log-tracer");
    let _ = global_span.enter();
    info!("Server started");
    let listener = TcpListener::bind(config.addr()).await?;
    let stats = statistics.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            info!(stats = format!("{}", stats.lock().await));
        }
    });
    loop {
        let (socket, socket_addr) = listener.accept().await?;
        let socket_span = span!(
            Level::TRACE,
            "socket-log-tracer",
            socket_addr = format!("{:?}", socket_addr)
        );
        let _guard = socket_span.enter();
        trace!("Socket connection accepted");
        let connection_config = config.clone();
        let connection_database = database.clone();
        let connection_statistics = statistics.clone();
        tokio::spawn(async {
            handle_connection(
                socket,
                connection_config,
                connection_database,
                connection_statistics,
            )
            .await
        });
    }
}

async fn handle_connection(
    mut source: TcpStream,
    config: Arc<Config>,
    database: Arc<Database>,
    statistics: Arc<Mutex<Statistics>>,
) -> Result<()> {
    let mut buff = [0u8; 1024];
    loop {
        let size = match source.read(&mut buff).await {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(e) => {
                error!(error = format!("{}", e));
                return Err(anyhow!("{}", e));
            }
        };

        let mut headers = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut headers);
        request.parse(&buff[..size])?;
        debug!(request = format!("{:?}", request));
        let request_method = request.method.unwrap();
        let request_path = request.path.unwrap();
        if request_method == "CONNECT" {
            match request
                .headers
                .iter()
                .find(|header| header.name == "Proxy-Authorization")
            {
                None => {
                    source
                        .write_all(b"HTTP/1.1 407 Proxy Authentication Required\r\n\r\n")
                        .await?;
                }
                Some(proxy_auth_header) => {
                    let (user, password) = parse_proxy_auth_token(proxy_auth_header.value)?;
                    if authenticate(&user, &password, &database) {
                        let mut target = TcpStream::connect(request_path).await?;
                        let (ingress, egress) = connect_target(
                            &mut source,
                            &mut target,
                            Duration::from_secs(config.connection_timeout),
                        )
                        .await?;
                        let mut stats = statistics.lock().await;
                        stats.add_ingress_traffic(&*user, ingress);
                        stats.add_egress_traffic(&*user, egress);
                    } else {
                        source
                            .write_all(b"HTTP/1.1 401 Unauthorized\r\n\r\n")
                            .await?;
                    }
                }
            }
        } else {
            source
                .write_all(b"HTTP/1.1 405 method Not Allowed\r\n\r\n")
                .await?;
        }

        source.write_all(b"").await?;
        return Ok(());
    }
}
async fn connect_target(
    source: &mut TcpStream,
    target: &mut TcpStream,
    timeout_sec: Duration,
) -> Result<(u64, u64)> {
    source
        .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await?;
    match timeout(timeout_sec, copy_bidirectional(source, target)).await {
        Ok(result) => {
            let (st, ts) = result?;
            info!(source_target = st, target_source = ts);
            Ok((st, ts))
        }
        Err(err) => {
            bail!(err)
        }
    }
}

fn parse_proxy_auth_token(token: &[u8]) -> Result<(String, String)> {
    let token_str = std::str::from_utf8(token)?;

    let encoded_cred = token_str
        .strip_prefix("Basic ")
        .ok_or_else(|| anyhow!("Invalid auth format: expected 'Basic ...'"))?;

    let decoded = general_purpose::STANDARD.decode(encoded_cred)?;
    let credentials = String::from_utf8(decoded)?;

    credentials
        .split_once(':')
        .map(|(u, p)| (u.to_string(), p.to_string()))
        .ok_or_else(|| anyhow!("Invalid credentials format: expected 'user:password'"))
}

fn authenticate(user: &str, password: &str, database: &Arc<Database>) -> bool {
    database.is_authenticated(user, password)
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_proxy() -> Result<()> {
        let task = tokio::spawn(async {
            run().await.expect("TODO: panic message");
        });
        let config = build_config();
        let mut socket = TcpStream::connect(config.addr()).await?;

        socket.write_all(b"hello world!").await?;
        let mut buff = [0u8; 1024];
        loop {
            let size = match socket.read(&mut buff).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    error!(error = format!("{}", e));
                    return Err(anyhow!("{}", e));
                }
            };
            let data = String::from_utf8_lossy(&buff[0..size]).to_string();
            assert_eq!(
                data,
                String::from("HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK")
            );
        }
        socket.write_all(b"").await?;
        task.await?;
        Ok(())
    }
}
