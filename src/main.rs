mod rocker;

extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate chrono;

use iron::prelude::*;
use router::Router;

use rocker::{info, ping, version};

fn main() {
    let mut router = Router::new();
    router.get("/info", info::handler);
    router.get("/_ping", ping::handler);
    router.get("/version", version::handler);

    Iron::new(router).http("localhost:2375").unwrap();
}
