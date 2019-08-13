mod commands;
mod github;

#[macro_use]
extern crate log;
extern crate env_logger;

use serde_json::Value;
use std::collections::HashMap;

use clap::{App, SubCommand};

type JsonArray = Vec<Value>;

fn main() {
    env_logger::init();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

    let matches = App::new("ggg: A Github CLI")
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .subcommand(
            SubCommand::with_name("notifications").about("Shows unread notifications count"),
        )
        .subcommand(
            SubCommand::with_name("list_notifications")
                .about("Shows unread notifications list per repo"),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("notifications") {
        commands::notifications::cmd();
    }

    if let Some(_matches) = matches.subcommand_matches("list_notifications") {
        let client = github::client().unwrap();
        let routes = github::v3::api_router::get_routes().unwrap();
        let notifications_url = routes.notifications_url;

        // TODO: This shouldn't go here, it is doing a request even when we're just getting the help.
        let mut resp = client.get(&notifications_url).send().unwrap();
        let nots: JsonArray = resp.json().unwrap();

        debug!("{:#?}", nots);
        info!("{:#?}", resp);

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
}
