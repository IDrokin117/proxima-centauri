use crate::auth::{authenticate, parse_proxy_auth_token, Database};
use crate::config::Config;
use crate::http_utils::response::ProxyResponse;
use crate::statistics::{LimitError, Limits, UsersStatistic};
use crate::tunnel::connect_target;
use anyhow::{bail, Result};
use httparse::{Request, EMPTY_HEADER};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, error, warn};

pub async fn handle_connection(
    mut source: TcpStream,
    config: Arc<Config>,
    database: Arc<Database>,
    statistics: Arc<Mutex<UsersStatistic>>,
) -> Result<()> {
    let mut buff = [0u8; 1024];

    let size = match source.read(&mut buff).await {
        Ok(0) => return Ok(()),
        Ok(n) => n,
        Err(e) => {
            error!(error = format!("{}", e));
            bail!(e);
        }
    };

    let mut headers = [EMPTY_HEADER; 16];
    let mut request = Request::new(&mut headers);
    request.parse(&buff[..size])?;

    debug!(request = format!("{:?}", request));
    let request_method = request.method.unwrap();
    let request_path = request.path.unwrap();

    if request_method != "CONNECT" {
        source
            .write_all(ProxyResponse::MethodNotAllowed.as_bytes())
            .await?;
        return Ok(());
    }
    let auth_header = request
        .headers
        .iter()
        .find(|header| header.name == "Proxy-Authorization");

    match auth_header {
        None => {
            source
                .write_all(ProxyResponse::ProxyAuthRequired.as_bytes())
                .await?;
        }
        Some(proxy_auth_header) => {
            let (user, password) = parse_proxy_auth_token(proxy_auth_header.value)?;

            if !authenticate(&user, &password, &database) {
                source
                    .write_all(ProxyResponse::Unauthorized.as_bytes())
                    .await?;
            }

            let mut user_stats = statistics.lock().await;
            user_stats.create_user(&user, Limits::with_low_limits());
            user_stats.inc_concurrency(&user);

            match user_stats.check_limits(&user) {
                Ok(_) => {
                    drop(user_stats);

                    let mut target = TcpStream::connect(request_path).await?;
                    let (ingress, egress) = connect_target(
                        &mut source,
                        &mut target,
                        Duration::from_secs(config.connection_timeout),
                    )
                    .await?;

                    let mut user_stats = statistics.lock().await;
                    user_stats.add_ingress_traffic(&user, ingress as u128);
                    user_stats.add_egress_traffic(&user, egress as u128);
                    user_stats.dec_concurrency(&user);
                }
                Err(err) => {
                    user_stats.dec_concurrency(&user);
                    drop(user_stats);

                    warn!(message = format!("{:?}", err));
                    match err {
                        LimitError::ConcurrencyLimitExceed(_) => {
                            source
                                .write_all(ProxyResponse::TooManyRequests.as_bytes())
                                .await?;
                        }
                        LimitError::TrafficLimitExceed(_) => {
                            source
                                .write_all(ProxyResponse::QuotaExceeded.as_bytes())
                                .await?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
