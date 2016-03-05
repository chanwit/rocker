use iron::mime::{Mime, TopLevel, SubLevel};

pub fn json_type() -> Mime {
    Mime(TopLevel::Application, SubLevel::Json, Vec::new())
}
