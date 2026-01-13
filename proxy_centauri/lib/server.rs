use crate::auth::Database;
use crate::config::{build_config, init};
use crate::handler::handle_connection;
use crate::statistics::Statistics;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{Level, info, span, trace};

pub struct Server {}

impl Server {
    pub async fn run() -> Result<()> {
        Self::run_on_addr(None).await
    }

    pub async fn run_on_addr(addr: Option<String>) -> Result<()> {
        init();
        let config = Arc::new(build_config());
        let bind_addr = addr.unwrap_or_else(|| config.addr());
        let database = Arc::new(Database::new_persistence());
        let statistics = Arc::new(Mutex::new(Statistics::new()));
        let global_span = span!(Level::TRACE, "global-log-tracer");
        let _ = global_span.enter();
        info!("Server started on {}", bind_addr);
        let listener = TcpListener::bind(&bind_addr).await?;
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
}
