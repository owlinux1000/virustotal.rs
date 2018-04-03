#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use std::collections::HashMap;
    
pub mod url;
pub mod api;
pub mod domain;

#[derive(Debug,Deserialize)]
pub struct ScanResponse {
    pub response_code: Option<u32>,
    pub verbose_msg: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct FileScan {
    pub detected: Option<bool>,
    pub version: Option<String>,
    pub result: Option<String>,
    pub update: Option<String>,
    pub detail: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct ReportResponse {
    pub response_code: u32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub filescan_id: Option<String>,
    pub positives: Option<u32>,
    pub total: Option<u32>,
    pub scans: Option<HashMap<String, FileScan>>
}

#[derive(Debug,Deserialize)]
pub struct Resolutions {
    pub last_resolved: String,
    pub ip_address: String
}

#[derive(Debug,Deserialize)]
pub struct DetectedUrls {
    pub url: String,
    pub positives: u32,
    pub total: u32,
    pub scan_date: String,
}

#[derive(Debug,Deserialize)]
pub struct DomainResponse {
    pub response_code: u32,
    pub verbose_msg: String,
    pub resolutions: Vec<Resolutions>,
    pub detected_urls: Vec<DetectedUrls>
}
