use anyhow::Result;
use proxy_centauri::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
