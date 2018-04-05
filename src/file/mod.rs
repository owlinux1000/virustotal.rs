use reqwest::Client;
use serde_json::from_str;
use super::*;

pub fn scan(api_key: &str, filename: &str) -> FileScanResponse {
    
    let form = reqwest::multipart::Form::new()
        .file("file", &filename)
        .expect("Not found");

    let mut resp = Client::new()
        .post(api::file::scan)
        .multipart(form)
        .form(&[("apikey", &api_key)])
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
    
}


   
