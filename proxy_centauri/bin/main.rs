use anyhow::Result;
use proxy_centauri::Server;

#[tokio::main]
async fn main() -> Result<()> {
    Server::run().await?;
    Ok(())
}
