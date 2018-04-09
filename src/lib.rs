#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use std::collections::HashMap;
    
pub mod url;
pub mod api;
pub mod domain;
pub mod ip;
pub mod file;
pub mod comment;

#[derive(Debug,Deserialize)]
pub struct PutCommentResponse {
    pub response_code: i32,
    pub verbose_msg: String
}

#[derive(Debug,Deserialize)]
pub struct Comment {
    pub date: String,
    pub comment: String
}

#[derive(Debug,Deserialize)]
pub struct GetCommentResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: String,
    pub comments: Vec<Comment>
}

#[derive(Debug,Deserialize)]
pub struct ScanResponse {
    pub response_code: Option<i32>,
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
pub struct FileScanResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub sha256: Option<String>,
    pub permalink: Option<String>
}

#[derive(Debug,Deserialize)]
pub struct FileReportResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub scan_date: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub url: Option<String>,
    pub permalink: Option<String>,
    pub filescan_id: Option<String>,
    pub positives: Option<u32>,
    pub total: Option<u32>,
    pub scans: Option<HashMap<String, FileScan>>
}

#[derive(Debug,Deserialize)]
pub struct ReportResponse {
    pub response_code: i32,
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
pub struct DomainResolutions {
    pub last_resolved: String,
    pub ip_address: String
}

#[derive(Debug,Deserialize)]
pub struct IpAddressResolutions {
    pub last_resolved: String,
    pub hostname: String
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
    pub response_code: i32,
    pub verbose_msg: String,
    pub resolutions: Vec<DomainResolutions>,
    pub detected_urls: Vec<DetectedUrls>
}

#[derive(Debug,Deserialize)]
pub struct IpAddressResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resolutions: Vec<IpAddressResolutions>,
    pub detected_urls: Vec<DetectedUrls>
}

#[derive(Debug,Deserialize)]
pub struct FileRescanResponse {
    pub response_code: i32,
    pub verbose_msg: String,
    pub resource: Option<String>,
    pub scan_id: Option<String>,
    pub permalink: Option<String>,
    pub sha256: Option<String>
}
