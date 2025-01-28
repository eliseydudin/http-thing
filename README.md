# http_router ⚙️
a simple library for routing!
```rust
struct PingPongRoute;

impl Route for PingPongRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/ping";

    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| Response::new().body(b"pong")
    }
}
```
and then create the server and add the route to it like this:
```rust
let mut server = Server::new(6060, 20);
server.add_route(PingPongRoute);
server.run()
```

# contributors
[eliseydudin](https://github.com/eliseydudin) 
[webbop](https://github.com/Webbopwork)
