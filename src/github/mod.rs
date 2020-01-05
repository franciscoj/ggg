mod credentials;
pub mod v3;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::{Client, Error};

// User agent is mandatory for github's API. Oterwise it responds with a 403
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub fn client() -> Result<Client, Error> {
    let headers = build_default_headers();

    Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()
}

pub fn build_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let token = format!("token {}", credentials::github_api_token());

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&token).unwrap(),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    headers
}
