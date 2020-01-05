use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GithubCredentials {
    github_api_token: String,
}

pub fn github_api_token() -> String {
    match envy::from_env::<GithubCredentials>() {
        Ok(creds) => creds.github_api_token,
        Err(err) => panic!("{:#?}", err),
    }
}
