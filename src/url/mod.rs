use reqwest::Client;
use serde_json::from_str;
use super::{VtClient, UrlScanResponse, UrlReportResponse};

impl <'a>VtClient<'a> {
    /// Scan an URL
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    /// 
    /// let vt = VtClient::new("Your API Key");
    /// let url = "https://example.com";
    /// vt.scan_url(url);
    /// ```
    pub fn scan_url(self, target_url: &str) -> UrlScanResponse {
        let url = &[self.endpoint, "/url/scan"].join("");
        let mut resp = Client::new()
            .post(url)
            .form(&[("apikey", self.api_key), ("url", target_url)])
            .send()
            .unwrap();
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
    }

    /// Retrieve URL scan reports
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    ///
    /// let vt = VtClient::new("Your API Key");
    /// vt.report_url(resource);
    /// ```
    pub fn report_url(self, resource: &str) -> UrlReportResponse {

        let params: &str = &format!(
            "?apikey={}&resource={}", self.api_key, &resource
        );
        let url = &[self.endpoint, "/url/report", params].join("");
        let mut resp = Client::new()
            .get(url)
            .send()
            .unwrap();
    
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
    }
}
