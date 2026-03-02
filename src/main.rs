mod cli;
mod config;
mod db;
mod handlers;
mod models;
mod server;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 运行CLI命令
    cli::run_cli().await?;

    Ok(())
}
