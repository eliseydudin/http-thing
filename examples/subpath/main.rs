mod addons;

use clang_log::init;
use http_thing::prelude::*;
use addons::parse_query;

struct SubPathRoute;

impl Route for SubPathRoute {
    fn path(&self) -> &'static str {
        "/"
    }

    fn request_type(&self) -> RequestType {
        RequestType::Get
    }

    fn handler(&mut self) -> fn(Request) -> Response {
        |req| {
            let query = parse_query(&req.query);
            if req.path == "/hello" || req.path.starts_with("/hello/") {
                Response::new()
                    .body(b"hello")
                    .header("content-type", "text/plain")
            } else {
                Response::new()
                    .body(format!("<!DOCTYPE html><html><head><meta http-equiv = \"refresh\" content = \"0; url = https://dempah.com/bigsearch/music/artists{}\"/></head></html>", req.fullpath).as_bytes())
                    .header("content-type", "text/html")
            }
        }
    }
}

fn main() {
    init(log::Level::max(), "subpath");

    let mut server = Server::default();
    server.add_default_handler(SubPathRoute);
    server.run();
}
