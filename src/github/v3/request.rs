use super::link_header;
use futures::executor::block_on;
use reqwest::{
    header::{HeaderMap, LINK},
    Client,
};
use serde_json::Value;

type JsonArray = Vec<Value>;

pub struct Request {
    url: String,
}

pub struct PageResponse {
    body: JsonArray,
    next_page_url: Option<String>,
}

impl Request {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn fetch_all(&self, client: &Client) -> JsonArray {
        let mut url_to_fetch = Some(self.url.to_owned());
        let mut items = vec![];

        while let Some(page_url) = url_to_fetch {
            let response = block_on(fetch_one(client, &page_url));

            items.extend(response.body);
            url_to_fetch = response.next_page_url;
        }

        items
    }
}

pub async fn fetch_one(client: &Client, url: &str) -> PageResponse {
    let request_builder = client.get(url);
    let response = request_builder.send().await.unwrap();
    let headers = response.headers();
    let next_page_url = get_next_page_url(headers);

    let body: JsonArray = response.json::<JsonArray>().await.unwrap();

    PageResponse {
        body,
        next_page_url,
    }
}

fn get_next_page_url(headers: &HeaderMap) -> Option<String> {
    info!("{:?}", headers);

    if let Some(link) = headers.get(LINK) {
        return link_header::parse(link.to_str().unwrap());
    }

    None
}
