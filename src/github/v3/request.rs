use reqwest::hyper_011::{
    header::{Link, LinkValue, RelationType},
    Headers,
};

use serde_json::Value;
use reqwest::Client;

type JsonArray = Vec<Value>;

pub struct Request {
    url: String
}

pub struct PageResponse{
    body: JsonArray,
    next_page_url: Option<String>,
}

impl Request {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn fetch_all(&self, client: &Client) -> JsonArray {
        let mut url_to_fetch = Some(self.url.to_owned());
        let mut nots = vec![];

        while let Some(page_url) = url_to_fetch {
            let response = fetch_one(client, &page_url);

            nots.extend(response.body);
            url_to_fetch = response.next_page_url;
        }

        nots
    }
}

pub fn fetch_one(client: &Client, url: &str) -> PageResponse {
    let mut resp = client.get(url).send().unwrap();

    debug!("{:#?}", resp);

    let headers = Headers::from(resp.headers().clone());
    let body: JsonArray = resp.json().unwrap();
    let next_page_url = get_next_page_url(headers);

    PageResponse {
        body,
        next_page_url,
    }
}

fn get_next_page_url(headers: Headers) -> Option<String> {
    if let Some(link) = headers.get::<Link>() {
        let mut iter = link.values().iter();

        if let Some(rel_next) = iter.find(is_rel_next) {
            Some(rel_next.link().to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

fn is_rel_next(link_value: &&LinkValue) -> bool {
    if let Some(rels_v) = link_value.rel() {
        rels_v.contains(&RelationType::Next)
    } else {
        false
    }
}
