mod routes;

use clang_log::init;
use http_thing::prelude::*;
use routes::*;

fn main() {
    init(log::Level::max(), "multiple");

    let mut server = Server::new(6060, 20);

    server.add_route(PingPongRoute);
    server.add_route(RatRoute);
    server.add_route(HeaderRoute);
    server.add_default_handler(LostRoute);

    server.run()
}
