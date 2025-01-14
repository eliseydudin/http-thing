# http_thing ⚙️
a simple library ive written to practise http stuff :3.
here you create a route like this:
```rust
struct PingPongRoute;

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
