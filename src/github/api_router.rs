const API_URL: &str = "https://api.github.com";

pub fn notifications() -> String {
    format!("{}/notifications", API_URL)
}
