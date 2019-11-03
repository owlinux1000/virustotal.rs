use reqwest::Client;
use serde_json::from_str;
use super::{VtClient, DomainReportResponse};

impl <'a>VtClient<'a> {
    /// Retrieves a domain report 
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    /// 
    /// let vt = VtClient::new("Your API Key");
    /// vt.repot_domain("alicemacs.com") # This is my domain :P
    /// ```
    pub fn repot_domain(self, domain: &'a str) -> DomainReportResponse {
        let params: &str = &format!(
            "?apikey={}&domain={}", self.api_key, &domain
        );
        let url = [self.endpoint, "/domain/report", params].join("");
        let mut resp = Client::new()
            .get(&url)
            .send()
            .unwrap();
        let text = resp.text().unwrap();
        from_str(&text).unwrap()
    }
}
