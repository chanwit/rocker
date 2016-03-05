use iron::prelude::*;
use iron::status;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}
