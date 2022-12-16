use std::collections::HashMap;
use std::env;
use std::fs;

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::header::USER_AGENT;

use tokio::time::{sleep, Duration};

use log::{info, warn};
use log4rs;

struct TelegramData {
    api: String,
    chat_id: String,
    message: String,
}

async fn get_new_version(url: &str) -> Result<String> {
    let client = reqwest::Client::new();

    for _ in 0..10 {
        let response = client
            .get(url)
            .header(USER_AGENT, "Just Random")
            .send()
            .await
            .context("Failed to send request")?;

        let response_json = json::parse(
            response
                .text()
                .await
                .context("Failed to parse response as a text")?
                .as_str(),
        )
        .context("Failed to parse response text as a json")?;
        match response_json["name"].as_str() {
            Some(version) => {
                return Ok(version.trim().to_string());
            }
            None => {
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    warn!("Couldn't get last version");
    Err(anyhow::anyhow!("Couldn't get last version"))
}

fn get_prev_version() -> Result<String> {
    fs::read_to_string("version.txt").context("Failed to read from file")
}

async fn notify(tg_data: &TelegramData) -> Result<()> {
    let client = reqwest::Client::new();

    let mut request_data = HashMap::new();
    request_data.insert("chat_id", tg_data.chat_id.clone());
    request_data.insert("text", tg_data.message.clone());

    client
        .post(format!(
            "https://api.telegram.org/bot{}/sendMessage",
            tg_data.api
        ))
        .json(&request_data)
        .send()
        .await
        .context("Failed to send telegram message")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    log4rs::init_file("log_config.yaml", Default::default())
        .context("Failed to load config for logging file")?;

    let api = env::var("TELEGRAM_BOT_API").context("Set up `TELEGRAM_BOT_API` first")?;
    let chat_id = env::var("TELEGRAM_CHAT_ID").context("Set up `TELEGRAM_CHAT_ID` first")?;

    let url = "https://api.github.com/repos/near/nearcore/releases/latest";
    let release_version_regex = Regex::new(r"^\d+\.\d+\.\d+$").context("Failed to create regex")?;

    if !std::path::Path::exists(std::path::Path::new("version.txt")) {
        fs::File::create("version.txt").context("Failed to create version.txt")?;
        info!("Created version.txt file");
    }

    let new_version = get_new_version(url).await?;

    if release_version_regex.is_match(&new_version) && new_version != get_prev_version()? {
        info!("New release version detected");

        fs::write("version.txt", new_version.as_bytes()).context("Failed to rewrite file")?;
        info!("Updated version.txt");

        notify(&TelegramData {
            api,
            chat_id,
            message: format!(
                "New version of Nearcore just came out!\n\nhttps://github.com/near/nearcore/releases/tag/{}",
                new_version
            ),
        })
        .await?;
        info!("Successfully sent telegram message");
    }

    Ok(())
}
