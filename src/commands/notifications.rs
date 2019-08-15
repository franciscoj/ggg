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
    let nots = fetch_all_pages(&client, &notifications_url);

    println!("{}", nots.len());
}

struct PageResponse {
    body: JsonArray,
    next_page_url: Option<String>,
}

fn fetch_page(client: &Client, page_url: &str) -> Option<PageResponse> {
    let mut resp = client.get(page_url).send().unwrap();
    let headers = Headers::from(resp.headers().clone());
    let body: JsonArray = resp.json().unwrap();

    if let Some(next_page_url) = get_next_page_url(headers) {
        Some(PageResponse {
            body,
            next_page_url: Some(next_page_url),
        })
    } else {
        Some(PageResponse {
            body,
            next_page_url: None,
        })
    }
}

fn fetch_all_pages(client: &Client, url: &str) -> JsonArray {
    let mut url_to_fetch = Some(url.to_owned());
    let mut nots = vec![];

    while let Some(ref page_url) = url_to_fetch {
        if let Some(fetched_page) = fetch_page(client, page_url) {
            nots.extend(fetched_page.body);
            url_to_fetch = None;

            if let Some(new_next_page_url) = fetched_page.next_page_url {
                url_to_fetch = Some(new_next_page_url);
            }
        }
    }

    nots
}

fn get_next_page_url(headers: Headers) -> Option<String> {
    if let Some(link) = headers.get::<Link>() {
        let values = link.values();
        let mut iter = values.iter();

        if let Some(rel_next) = iter.find(is_rel_next) {
            let link = rel_next.link();

            Some(link.to_owned())
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
