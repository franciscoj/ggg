mod credentials;

#[macro_use]
extern crate log;
extern crate env_logger;

use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::collections::HashMap;

type JsonArray = Vec<Value>;

fn main() {
    env_logger::init();

    let token = credentials::github_api_token();
    let client = Github::new(token).unwrap();

    let (headers, status, json) = client.get().notifications().execute::<JsonArray>().unwrap();

    debug!("Status: {:#?}", status);
    debug!("Headers: {:#?}", headers);

    if let Some(nots) = json {
        debug!("{:#?}", nots);
        let mut repos_map: HashMap<String, i32> = HashMap::new();

        for n in nots.iter() {
            let repo_name = n["repository"]["full_name"].to_string();

            let nots_count = repos_map.entry(repo_name).or_insert(0);
            *nots_count += 1;

            info!(
                "[{repo} | {reason}] {title}",
                repo = n["repository"]["full_name"],
                reason = n["reason"],
                title = n["subject"]["title"]
            );
        }

        debug!("{:#?}", nots[0]);

        println!("Total notifications: {}", nots.len());
        for (repo, nots_count) in &repos_map {
            println!("Notifications {}: {}", repo, nots_count);
        }
    }
}
