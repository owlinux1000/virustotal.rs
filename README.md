# VirusTotal.rs

[![Build Status](https://travis-ci.org/owlinux1000/virustotal.rs.svg?branch=master)](https://travis-ci.org/owlinux1000/virustotal.rs)

Library for VirusTotal API

## Implemented Features

| Resource                        | Description                        | Permission |
|:--------------------------------|:-----------------------------------|:-----------|
| GET /vtapi/v2/file/report       | Retrieve file scan reports         | public     |
| POST /vtapi/v2/file/scan        | Upload and scan a file             | public     |
| POST /vtapi/v2/file/rescan      | Rescanning already submitted files | public     |
| GET /vtapi/v2/url/report        | Retrieve URL scan reports          | public     |
| POST /vtapi/v2/url/scan         | Scan an URL                        | public     |
| POST /vtapi/v2/comments/put     | Make comments on files and URLs    | public     |
| GET /vtapi/v2/domain/report     | Retrieves a domain report          | public     |
| GET /vtapi/v2/ip-address/report | Retrieve an IP address report      | public     |

## Example

```
extern crate virustotal;

use virustotal::*;

fn main() {

    let api = "Your API KEY";
    let url = "The URL you want to check";
    let res = url::scan(api, url);
    println!("{:?}", url::report(api, &res.scan_id.unwrap()));
    
}
```

