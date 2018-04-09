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

pub mod file {
    pub static scan: &str = "https://www.virustotal.com/vtapi/v2/file/scan";
    pub static rescan: &str = "https://www.virustotal.com/vtapi/v2/file/rescan";
    pub static report: &str = "https://www.virustotal.com/vtapi/v2/file/report";
}

pub mod comment {
    pub static put: &str = "https://www.virustotal.com/vtapi/v2/comments/put";
    pub static get: &str = "https://www.virustotal.com/vtapi/v2/comments/get";
}
