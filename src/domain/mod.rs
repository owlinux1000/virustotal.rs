use reqwest::Client;
use serde_json::from_str;
use super::*;

pub fn report(api_key: &str, domain: &str) -> DomainResponse {

    let params: &str = &format!("?apikey={}&domain={}", &api_key, &domain);
    let url = [api::domain::report, params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
}
