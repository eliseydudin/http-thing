use clang_log::init;
use http_router::prelude::*;

struct PingPongRoute;

impl Route for PingPongRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/ping";

    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| Response::new().body(b"pong")
    }
}

fn main() {
    init(log::Level::Trace, "pingpong");
    let mut server = Server::new(8080, 20);
    server.add_route(PingPongRoute);

    server.run()
}
