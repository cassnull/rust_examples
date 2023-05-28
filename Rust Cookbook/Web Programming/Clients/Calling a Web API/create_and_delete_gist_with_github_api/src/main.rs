use error_chain::error_chain;
use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Login {
    #[serde(rename = "oauth")]
    OAuth(String),
    #[serde(rename = "personal_access_token")]
    PersonalAccessToken { username: String, token: String },
}

trait RequestBuilder {
    fn auth(self, login: &Login) -> Self;
}

impl RequestBuilder for reqwest::RequestBuilder {
    fn auth(self, login: &Login) -> Self {
        match login {
            Login::OAuth(token) => self.header(AUTHORIZATION, format!("token {}", token)),
            Login::PersonalAccessToken { username, token } => {
                self.basic_auth(username, Some(token))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let username = env::var("GH_USER")?;
    let token = env::var("GH_PASS")?;

    let login = Login::PersonalAccessToken { username, token };

    let gist_body = json!({
    "description": "the description for this gist",
    "public": true,
    "files": {
         "main.rs": {
         "content": r#"fn main() { println!("hello world!");}"#
        }
    }});

    let user_agent =
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";

    let request_url = "https://api.github.com/gists";
    let client = reqwest::Client::builder().user_agent(user_agent).build()?;
    let response = client
        .post(request_url)
        .auth(&login)
        .json(&gist_body)
        .send()
        .await?;

    let gist: Gist = response.json().await?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}", request_url, gist.id);
    let client = reqwest::Client::builder().user_agent(user_agent).build()?;
    let response = client.delete(&request_url).auth(&login).send().await?;

    println!(
        "Gist {} deleted! Status code: {}",
        gist.id,
        response.status()
    );
    Ok(())
}
