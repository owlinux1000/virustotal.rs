use reqwest::Client;
use serde_json::from_str;
use super::*;

pub fn scan(api_key: &str, url: &str) -> ScanResponse {
    
    let mut resp = Client::new()
        .post(api::url::scan)
        .form(&[("apikey", &api_key), ("url", &url)])
        .send()
        .unwrap();
    
    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
        
}

pub fn report(api_key: &str, resource: &str) -> ReportResponse {

    let params: &str = &format!("?apikey={}&resource={}", &api_key, &resource);
    let url = [api::url::report, params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
}


