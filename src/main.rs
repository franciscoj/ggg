mod credentials;

#[macro_use]
extern crate log;
extern crate env_logger;

use github_rs::client::{Executor, Github};
use serde_json::Value;

fn main() {
    env_logger::init();

    let token = credentials::github_api_token();
    let client = Github::new(token).unwrap();

    // TODO: This can for sure be improved to use a better type than `Value`
    let notifications = client.get().notifications().execute::<Value>();

    match notifications {
        Ok((headers, status, json)) => {
            info!("Status: {:#?}", status);
            info!("Headers: {:#?}", headers);

            if let Some(json) = json {
                if let Value::Array(arr) = json {
                    debug!("{:#?}", arr);
                    println!("Notifications: {}", arr.len());
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}
