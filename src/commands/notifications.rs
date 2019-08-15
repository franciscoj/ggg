use crate::github;
use reqwest::hyper_011::{
    header::{Link, LinkValue, RelationType},
    Headers,
};
use reqwest::Client;
use serde_json::Value;

type JsonArray = Vec<Value>;

pub fn cmd() {
    let client = github::client().unwrap();
    let routes = github::v3::api_router::get_routes().unwrap();
    let notifications_url = routes.notifications_url;
    let nots = fetch_all(&client, &notifications_url);

    println!("{}", nots.len());
}

struct PageResponse {
    body: JsonArray,
    next_page_url: Option<String>,
}

fn fetch_one(client: &Client, page_url: &str) -> PageResponse {
    let mut resp = client.get(page_url).send().unwrap();
    let headers = Headers::from(resp.headers().clone());
    let body: JsonArray = resp.json().unwrap();
    let next_page_url = get_next_page_url(headers);

    PageResponse {
        body,
        next_page_url,
    }
}

fn fetch_all(client: &Client, url: &str) -> JsonArray {
    let mut url_to_fetch = Some(url.to_owned());
    let mut nots = vec![];

    while let Some(ref page_url) = url_to_fetch {
        let response = fetch_one(client, page_url);

        nots.extend(response.body);
        url_to_fetch = response.next_page_url;
    }

    nots
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
