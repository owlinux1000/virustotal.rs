use reqwest::Client;
use serde_json::from_str;
use super::*;

pub fn scan(api_key: &str, filename: &str) -> FileScanResponse {
    
    let form = reqwest::multipart::Form::new()
        .text("apikey".to_string(), api_key.to_string())
        .file("file", &filename)
        .expect("Not found");
    
    let mut resp = Client::new()
        .post(api::file::scan)
        .multipart(form)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
    
}

pub fn report(api_key: &str, resource: &str) -> FileReportResponse {
    
    let params: &str = &format!("?apikey={}&resource={}", &api_key, &resource);
    let url = [api::file::report, params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();
    
    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()

}
