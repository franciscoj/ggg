use serde::Deserialize;
use serde_json::Error;

#[derive(Deserialize, Debug)]
pub struct Endpoints {
    pub notifications_url: String,
}

pub fn get_routes() -> Result<Endpoints, Error> {
    let routes = include_str!("api_routes.json");

    serde_json::from_str(&routes)
}
