use clang_log::init;
use http_thing::prelude::*;

pub struct PingPongRoute;

impl Route for PingPongRoute {
    fn path(&self) -> &'static str {
        "/ping"
    }

    fn request_type(&self) -> RequestType {
        RequestType::Get
    }

    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| {
            Response::new()
                .body(b"pong".to_vec())
                .header("content-type", "text/plain")
        }
    }
}

fn main() {
    init(log::Level::max(), "pong");

    let mut server = Server::new(6060, 20);
    server.add_route(PingPongRoute);
    server.run()
}
