use reqwest::Client;
use serde_json::from_str;
use super::*;

/// Retrieves a domain report 
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let domain = "the domain you want to check";
/// domain::report(api_key, domain);
/// ```
pub fn report(api_key: &str, domain: &str) -> DomainReportResponse {

    let params: &str = &format!("?apikey={}&domain={}", &api_key, &domain);
    let url = ["https://www.virustotal.com/vtapi/v2/domain/report", params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
}
