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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_notifications_url() {
        let routes = get_routes().unwrap();

        assert_eq!(
            routes.notifications_url,
            "https://api.github.com/notifications"
        )
    }
}
