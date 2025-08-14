use anyhow::Result;
use clap::Parser;

use crate::{cli::Cli, webdriver::start_web_driver};

mod api;
mod cli;
mod lua;
mod webdriver;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let (mut child, driver) = start_web_driver(args.browser).await?;
    let reuslt = lua::exec_lua(args.file_path, driver).await;
    let _ = child.kill().await;
    reuslt?;
    Ok(())
}
