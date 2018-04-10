use reqwest::Client;
use serde_json::from_str;
use super::*;

/// Upload and scan a file
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let path = "the file path you want to scan";
/// file::scan(api_key, path);
/// ```
pub fn scan(api_key: &str, filename: &str) -> FileScanResponse {
    
    let form = reqwest::multipart::Form::new()
        .text("apikey".to_string(), api_key.to_string())
        .file("file", &filename)
        .expect("Not found");
    
    let mut resp = Client::new()
        .post("https://www.virustotal.com/vtapi/v2/file/scan")
        .multipart(form)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
    
}

/// Retrieve file scan reports
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let resource = "the resource you want to check";
/// file::report(api_key, resource);
/// ```
pub fn report(api_key: &str, resource: &str) -> FileReportResponse {
    
    let params: &str = &format!("?apikey={}&resource={}", &api_key, &resource);
    let url = ["https://www.virustotal.com/vtapi/v2/file/report", params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();
    
    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()

}

/// Rescanning already submitted files
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let resource = "the resource you want to check";
/// file::report(api_key, resource);
/// ```
pub fn rescan(api_key: &str, resource: &str) -> FileRescanResponse {
    
    let mut resp = Client::new()
        .post("https://www.virustotal.com/vtapi/v2/file/rescan")
        .form(&[("apikey", &api_key), ("resource", &resource)])
        .send()
        .unwrap();
    
    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()

}
