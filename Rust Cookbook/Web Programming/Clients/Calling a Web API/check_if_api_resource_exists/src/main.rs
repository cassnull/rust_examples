use reqwest::Result;
use reqwest::{header::USER_AGENT, ClientBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let user = "cassnull";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let user_agent =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client
        .head(&request_url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
