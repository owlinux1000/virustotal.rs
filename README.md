# VirusTotal.rs

[![Build Status](https://travis-ci.org/owlinux1000/virustotal.rs.svg?branch=master)](https://travis-ci.org/owlinux1000/virustotal.rs)

Library for VirusTotal API

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


