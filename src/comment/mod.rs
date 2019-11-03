//use std::collections::HashMap;
use reqwest::Client;
use serde_json::from_str;
use super::{VtClient, CommentGetResponse, CommentPutResponse};

impl <'a>VtClient<'a> {
    /// Make comments on file and URLs
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    /// 
    /// let vt = VtClient::new("Your API Key");
    /// let resource = "the resource you want to put comments";
    /// let comment = "This is a test";
    /// vt.put(resource, comment);
    /// ```
    pub fn put(self, resource: &str, comment: &str) -> CommentPutResponse {
        let url = &[self.endpoint, "/comments/put"].join("");
        let mut resp = Client::new()
            .post(url)
            .form(&[("apikey", self.api_key), ("comment", &comment), ("resource", &resource)])
            .send()
            .unwrap();
    
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
    }
    
    /// Get comments for a file or URL
    ///
    /// # Example
    /// 
    /// ```
    /// use virustotal::*;
    /// 
    /// let vt = VtClient::new("Your API Key");
    /// let resource = "the resource you want to get comments";
    /// vt.get(comment);
    /// ```
    pub fn get(self, resource: &str) -> CommentGetResponse {
        let params: &str = &format!(
            "?apikey={}&resource={}", self.api_key, &resource
        );
        let url = &[self.endpoint, "/comments/get", params].join("");
        let mut resp = Client::new()
            .get(url)
            .send()
            .unwrap();
        let text: &str = &resp.text().unwrap();
        from_str(&text).unwrap()
    }
}
