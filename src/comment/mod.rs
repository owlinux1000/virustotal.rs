use reqwest::Client;
use serde_json::from_str;
use super::*;

/// Make comments on file and URLs
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let resource = "the resource you want to put comments";
/// let comment = "This is a test";
/// comment::put(api_key, resource, comment);
/// ```
pub fn put(api_key: &str, resource: &str, comment: &str) -> PutCommentResponse {
    
    let mut resp = Client::new()
        .post(api::comment::put)
        .form(&[("apikey", &api_key), ("comment", &comment), ("resource", &resource)])
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


