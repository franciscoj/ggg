pub mod credentials;
pub mod api_router;

use reqwest::header;
use reqwest::header::HeaderMap;
use reqwest::{Client, Error};

pub fn client() -> Result<Client, Error> {
    let headers = get_headers();

    Client::builder().default_headers(headers).build()
}

fn get_headers() -> HeaderMap {
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
