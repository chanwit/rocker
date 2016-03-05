use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use chrono::UTC;

use rocker::util::*;
use rocker::types::*;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    let version = Version {
        Version: "1.11.0-dev".to_string(),
        Os: "linux".to_string(),
        KernelVersion: "3.19.0-23-generic".to_string(),
        GoVersion: "rust1.7.0".to_string(),
        GitCommit: "N/A".to_string(),
        Arch: "amd64".to_string(),
        ApiVersion: "1.23".to_string(),
        BuildTime: UTC::now().to_rfc3339(),
        Experimental: true,
    };

    let encoded = json::encode(&version).unwrap();

    // String case to &str automatically
    Ok(Response::with((json_type(), status::Ok, encoded)))
}
