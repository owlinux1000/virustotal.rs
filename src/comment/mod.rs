use reqwest::Client;
use serde_json::from_str;
use super::*;

pub fn put(api_key: &str, comment: &str) -> PutCommentResponse {
    
    let mut resp = Client::new()
        .post(api::comment::put)
        .form(&[("apikey", &api_key), ("put", &comment)])
        .send()
        .unwrap();
    
    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
        
}

pub fn get(api_key: &str, resource: &str) -> GetCommentResponse {

    let params: &str = &format!("?apikey={}&resource={}", &api_key, &resource);
    let url = [api::comment::get, params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
}


