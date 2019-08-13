use serde_json::Value;
use crate::github;

type JsonArray = Vec<Value>;

pub fn cmd() {
    let client = github::client().unwrap();
    let routes = github::v3::api_router::get_routes().unwrap();
    let notifications_url = routes.notifications_url;

    let mut resp = client.get(&notifications_url).send().unwrap();
    let nots: JsonArray = resp.json().unwrap();

    debug!("{:#?}", nots);
    info!("{:#?}", resp);

    println!("{}", nots.len());
}
