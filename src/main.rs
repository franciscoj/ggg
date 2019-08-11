mod credentials;

use github_rs::client::{Executor, Github};
use serde_json::Value;

fn main() {
    let token = credentials::github_api_token();
    let client = Github::new(token).unwrap();
    let notifications = client.get().notifications().execute::<Value>();

    match notifications {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                if let Value::Array(arr) = json {
                    // println!("{:#?}", arr);
                    println!("Notifications: {}", arr.len());
                }
            }
        }
        Err(e) => println!("{}", e),
    }
}
