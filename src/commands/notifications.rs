use crate::github::{self, v3::request::Request};

pub fn cmd() {
    let client = github::client().unwrap();
    let routes = github::v3::api_router::get_routes().unwrap();
    let notifications_url = routes.notifications_url;
    let req = Request::new(notifications_url);
    let nots = req.fetch_all(&client);

    println!("{}", nots.len());
}
