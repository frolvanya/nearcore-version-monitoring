use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use anyhow::{Context, Result};
use regex::Regex;
use reqwest::header::USER_AGENT;

use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

use tokio::time::{sleep, Duration};

struct EmailData<'a> {
    smtp: &'a str,
    recipient: &'a str,
    hostname: &'a str,
    password: &'a str,
    message: &'a str,
}

async fn get_new_version(url: &str, new_version: &mut String) -> Result<()> {
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
                new_version.push_str(version.trim());
                break;
            }
            None => {
                new_version.clear();
                sleep(Duration::from_secs(5)).await;
            }
        }
    }

    if new_version.is_empty() {
        return Err(anyhow::anyhow!("New version is None value"));
    }

    Ok(())
}

fn get_prev_version() -> Result<String> {
    let mut versions_file = File::open("version.txt").context("Failed to open file")?;
    let mut buffer = String::new();

    versions_file
        .read_to_string(&mut buffer)
        .context("Failed to read from file")?;

    Ok(buffer)
}

fn notify(email_data: &EmailData) -> Result<()> {
    let email = EmailBuilder::new()
        .to(email_data.recipient)
        .from(email_data.hostname)
        .subject("New version of Nearcore just came")
        .text(email_data.message)
        .build()
        .context("Failed to build email")?;

    let mut mailer = SmtpClient::new_simple(email_data.smtp)
        .context("Failed to create new SMTP client")?
        .credentials(Credentials::new(
            email_data.hostname.to_string(),
            email_data.password.to_string(),
        ))
        .transport();

    mailer.send(email.into()).context("Failed to send email")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let smtp_server = env::var_os("SMTP_SERVER").unwrap_or_else(|| {
        println!("Set up `SMTP_SERVER` first");
        std::process::exit(1);
    });
    let email_recipient = env::var_os("EMAIL_RECIPIENT").unwrap_or_else(|| {
        println!("Set up `EMAIL_RECIPIENT` first");
        std::process::exit(1);
    });
    let email_hostname = env::var_os("EMAIL_HOSTNAME").unwrap_or_else(|| {
        println!("Set up `EMAIL_HOSTNAME` first");
        std::process::exit(1);
    });
    let email_password = env::var_os("EMAIL_PASSWORD").unwrap_or_else(|| {
        println!("Set up `EMAIL_PASSWORD` first");
        std::process::exit(1);
    });

    let url = "https://api.github.com/repos/near/nearcore/releases/latest";
    let release_version_regex = Regex::new(r"^\d+\.\d+\.\d+$").context("Failed to create regex")?;

    if !std::path::Path::exists(&std::path::Path::new("version.txt")) {
        File::create("version.txt").context("Failed to create version.txt")?;
    }

    let mut new_version = String::new();
    get_new_version(url, &mut new_version).await?;

    if release_version_regex.is_match(&new_version) && new_version != get_prev_version()? {
        let mut versions_file = OpenOptions::new()
            .write(true)
            .open("version.txt")
            .context("Failed to open file for writing")?;
        versions_file
            .write(new_version.as_bytes())
            .context("Failed to rewrite file")?;

        notify(&EmailData {
            smtp: smtp_server
                .to_str()
                .context("Failed to unwrap `smpt_server`")?,
            recipient: email_recipient
                .to_str()
                .context("Failed to unwrap `email_recipient`")?,
            hostname: email_hostname
                .to_str()
                .context("Failed to unwrap `email_hostname`")?,
            password: email_password
                .to_str()
                .context("Failed to unwrap `email_password`")?,
            message: format!(
                "https://github.com/near/nearcore/releases/tag/{}",
                new_version
            )
            .as_str(),
        })?;
    }

    Ok(())
}
