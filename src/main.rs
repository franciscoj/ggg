mod credentials;

#[macro_use]
extern crate log;
extern crate env_logger;

use github_rs::client::{Executor, Github};
use github_rs::errors::Error as GithubError;
use github_rs::{HeaderMap, StatusCode};

use serde_json::Value;
use std::collections::HashMap;

type JsonArray = Vec<Value>;

fn new_client() -> Result<Github, GithubError> {
    let token = credentials::github_api_token();
    Github::new(token)
}

fn my_notifications(
    client: &Github,
) -> Result<(HeaderMap, StatusCode, Option<JsonArray>), GithubError> {
    client.get().notifications().execute::<JsonArray>()
}

fn main() {
    env_logger::init();

    let client = new_client().unwrap();
    let (headers, status, json) = my_notifications(&client).unwrap();

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
