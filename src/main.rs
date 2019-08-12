mod github;

#[macro_use]
extern crate log;
extern crate env_logger;

use serde_json::Value;
use std::collections::HashMap;

type JsonArray = Vec<Value>;

fn main() {
    env_logger::init();

    let client = github::client().unwrap();
    let routes = github::v3::api_router::get_routes().unwrap();
    let notifications_url = routes.notifications_url;

    let mut resp = client.get(&notifications_url).send().unwrap();
    let nots: JsonArray = resp.json().unwrap();

    debug!("{:#?}", nots);
    info!("{:#?}", resp);

    let mut repos_map: HashMap<String, i32> = HashMap::new();

    for n in nots.iter() {
        let repo_name = n["repository"]["full_name"].to_string();

        let nots_count = repos_map.entry(repo_name).or_insert(0);
        *nots_count += 1;
    }

    println!("Total notifications: {}", nots.len());

    for (repo, nots_count) in &repos_map {
        println!("Notifications {}: {}", repo, nots_count);
    }
}
