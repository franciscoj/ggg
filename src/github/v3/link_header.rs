use regex::Regex;

pub fn parse(link: &str) -> Option<String> {
    let regex = Regex::new(r#".*<(.*)>; rel="next".*"#).unwrap();
    let maybe_captures = regex.captures(link);

    if let Some(captures) = maybe_captures {
        return Some(captures.get(1).unwrap().as_str().to_owned().to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_link_url() {
        let link = "<https://api.github.com/notifications?page=2>; rel=\"next\", <https://api.github.com/notifications?page=2>; rel=\"last\"";
        let url = parse(link).unwrap();

        assert_eq!(url, "https://api.github.com/notifications?page=2")
    }

    #[test]
    fn it_parses_empty_link() {
        let link = "";
        let url = parse(link);

        assert_eq!(url, None)
    }
}
