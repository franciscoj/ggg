use crate::github::{self, v3::request::Request};
use std::collections::HashMap;

pub fn cmd() {
    let client = github::client().unwrap();
    let routes = github::v3::api_router::get_routes().unwrap();
    let notifications_url = routes.notifications_url;
    let req = Request::new(notifications_url);
    let nots = req.fetch_all(&client);
    let mut repos_map: HashMap<String, i32> = HashMap::new();

    println!("Total notifications: {}", nots.len());

    for n in nots.iter() {
        let repo_name = n["repository"]["full_name"].to_string();

        let nots_count = repos_map.entry(repo_name).or_insert(0);
        *nots_count += 1;
    }

    for (repo, nots_count) in &repos_map {
        println!("Notifications {}: {}", repo, nots_count);
    }
}
