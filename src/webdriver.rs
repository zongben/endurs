use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::process::{Child, Command};

use crate::cli::Browser;

async fn wait_for_driver(port: String) -> Result<()> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:{}/status", port);
    for _ in 0..50 {
        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    anyhow::bail!("Driver did not respond in time");
}

pub async fn start_web_driver(browser: Browser, port: String) -> Result<(Child, WebDriver)> {
    let arg = format!("--port={}", port);
    let host = format!("http://localhost:{}", port);

    let command = match browser {
        Browser::Chrome => "chromedriver",
        Browser::Firefox => "geckodriver",
    };
    let child = Command::new(command).arg(arg).spawn()?;

    wait_for_driver(port).await?;

    let driver = match browser {
        Browser::Chrome => WebDriver::new(host, DesiredCapabilities::chrome()).await?,
        Browser::Firefox => WebDriver::new(host, DesiredCapabilities::firefox()).await?,
    };

    Ok((child, driver))
}
