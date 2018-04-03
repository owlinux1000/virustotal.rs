pub mod url {
    pub static scan: &str = "https://www.virustotal.com/vtapi/v2/url/scan";
    pub static report: &str = "https://www.virustotal.com/vtapi/v2/url/report";
}

pub mod domain {
    pub static report: &str = "https://www.virustotal.com/vtapi/v2/domain/report";
}

pub mod ip {
    pub static report: &str = "https://www.virustotal.com/vtapi/v2/ip-address/report";
}
