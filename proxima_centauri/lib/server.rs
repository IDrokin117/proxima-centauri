use crate::auth::Database;
use crate::config::{build_config, init};
use crate::handler::handle_connection;
use crate::statistics::UsersStatistic;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{debug, info, span, Level};

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
        let user_stats = Arc::new(Mutex::new(UsersStatistic::new()));
        let global_span = span!(Level::TRACE, "global-log-tracer");
        let _ = global_span.enter();
        let stats = user_stats.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                let stats_guard = stats.lock().await;
                if !stats_guard.is_empty() {
                    info!(stats = format!("{}", stats_guard));
                }
            }
        });
        info!("Server started on {}", bind_addr);
        let listener = TcpListener::bind(&bind_addr).await?;

        loop {
            let (socket, socket_addr) = listener.accept().await?;
            let socket_span = span!(
                Level::TRACE,
                "socket-log-tracer",
                socket_addr = format!("{:?}", socket_addr)
            );
            let _guard = socket_span.enter();
            debug!("Socket connection accepted {socket_addr}");
            let connection_config = config.clone();
            let connection_database = database.clone();
            let connection_statistics = user_stats.clone();
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
