use reqwest::Client;
use serde_json::from_str;
use super::*;

impl <'a>VtClient<'a> {
    /// Upload and scan a file
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    ///
    /// let vt = VtClient::new("Your API Key");
    /// vt.scan_file("eicar.txt")
    /// ```
    pub fn scan_file(self, filename: &str) -> FileScanResponse {
    
        let form = reqwest::multipart::Form::new()
            .text("apikey".to_string(), self.api_key.to_string())
            .file("file", filename)
            .expect("Not found");

        let url = &[self.endpoint, "/file/scan"].join("");
        let mut resp = Client::new()
            .post(url)
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
    /// let vt = VtClient::new("Your API Key");
    /// vt.report_file("eicar.txt")
    /// ```
    pub fn report_file(self, resource: &str) -> FileReportResponse {
        
        let params: &str = &format!(
            "?apikey={}&resource={}", self.api_key, &resource
        );
        let url = &[self.endpoint, "/file/report", params].join("");
        let mut resp = Client::new()
            .get(url)
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
    /// let vt = VtClient::new("Your API Key");
    /// vt.rescan(resource);
    /// ```
    pub fn rescan_file(self, resource: &str) -> FileRescanResponse {

        let url = &[self.endpoint, "/file/rescan"].join("");
        let mut resp = Client::new()
            .post(url)
            .form(&[("apikey", self.api_key), ("resource", &resource)])
            .send()
            .unwrap();
        
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
     
    }
}
