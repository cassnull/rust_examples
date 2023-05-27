use std::time::Duration;

use anyhow::Result;
use reqwest::{header::USER_AGENT, ClientBuilder};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    let user_agent =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";

    let timeout = Duration::from_secs(5);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client
        .get(&request_url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}
