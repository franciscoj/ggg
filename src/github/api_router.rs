use serde::Deserialize;
use serde_json::Error;
use std::fs;

const ENDPOINTS: &str = "endpoints.json";

#[derive(Deserialize, Debug)]
pub struct Endpoints {
    pub notifications_url: String,
}

pub fn get_routes() -> Result<Endpoints, Error> {
    let routes_json = fs::read_to_string(ENDPOINTS).unwrap();

    serde_json::from_str(&routes_json)
}
