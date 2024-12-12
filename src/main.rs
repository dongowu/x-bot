mod github;
mod x_api;

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let mut github_handler = github::GitHubHandler::new(
        "xxxxx".to_string(),
        "dongowu".to_string(),
        "project_rust".to_string(),
    ).await?;
    
    let x_handler = x_api::XHandler::new("xxxf".to_string())?;

    loop {
        if let Some(contributor) = github_handler.check_new_contributor().await? {
            x_handler.post_new_contributor(&contributor).await?;
        }

        if let Some(release) = github_handler.check_new_release().await? {
            x_handler.post_new_release(&release).await?;
        }

        sleep(Duration::from_secs(300)).await; // 每5分钟检查一次
    }
}
