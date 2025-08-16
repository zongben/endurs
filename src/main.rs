use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

mod api;
mod cli;
mod lua;
mod test_runner;
mod webdriver;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    lua::exec_lua(args.file_path, args.browser, args.port).await?;

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    Ok(())
}
