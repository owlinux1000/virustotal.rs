use reqwest::Client;
use serde_json::from_str;
use super::{VtClient, IpAddressReportResponse};
impl <'a>VtClient<'a> {
    /// Retrieve an IP address report
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    /// 
    /// let vt = VtClient::new("Your API Key");
    /// vt.report_ip_address("192.168.2.1");
    /// ```
    pub fn report_ip_address(self, ip_address: &str) -> IpAddressReportResponse {
        let params: &str = &format!(
            "?apikey={}&ip={}", self.api_key, &ip_address
        );
        let url = [self.endpoint, "/ip-address/report", params].join("");
        let mut resp = Client::new()
            .get(&url)
            .send()
            .unwrap();
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
    }            
}
