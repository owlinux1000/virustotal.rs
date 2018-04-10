use reqwest::Client;
use serde_json::from_str;
use super::*;

/// Retrieve an IP address report
///
/// # Example
/// 
/// ```
/// use virustotal::*;
/// 
/// let api_key = "Your API key";
/// let ip_address = "the IP address you want to check";
/// ip::report(api_key, domain);
/// ```
pub fn report(api_key: &str, ip_address: &str) -> IpAddressResponse {

    let params: &str = &format!("?apikey={}&ip={}", &api_key, &ip_address);
    let url = ["https://www.virustotal.com/vtapi/v2/ip-address/report", params].join("");
    let mut resp = Client::new()
        .get(&url)
        .send()
        .unwrap();

    let text: &str = &resp.text().unwrap();
    from_str(&text).unwrap()
}
