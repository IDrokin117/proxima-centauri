use crate::http_utils::response::ProxyResponse;
use anyhow::{bail, Result};
use std::time::Duration;
use tokio::io::{copy_bidirectional, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
pub async fn connect_target(
    source: &mut TcpStream,
    target: &mut TcpStream,
    timeout_sec: Duration,
) -> Result<(u64, u64)> {
    source
        .write_all(ProxyResponse::ConnectionEstablished.as_bytes())
        .await?;

    match timeout(timeout_sec, copy_bidirectional(source, target)).await {
        Ok(result) => {
            let (st, ts) = result?;
            Ok((st, ts))
        }
        Err(err) => {
            bail!(err)
        }
    }
}
