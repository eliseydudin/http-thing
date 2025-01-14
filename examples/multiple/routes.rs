use http_thing::prelude::*;

pub struct PingPongRoute;
pub struct RatRoute;
pub struct LostRoute;
pub struct HeaderRoute;

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

impl Route for RatRoute {
    fn path(&self) -> &'static str {
        "/rat"
    }

    fn request_type(&self) -> RequestType {
        RequestType::Get
    }

    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| {
            Response::new()
                .body(include_bytes!("html/rat.html").to_vec())
                .header("content-type", "text/html")
        }
    }
}

impl Route for LostRoute {
    fn path(&self) -> &'static str {
        "/"
    }

    fn request_type(&self) -> RequestType {
        RequestType::Get
    }

    fn handler(&mut self) -> fn(Request) -> Response {
        |req| {
            Response::new()
                .body(
                    format!(
                        include_str!("html/404.html"),
                        req.path, req.addr, req.data, req.headers, req.fullpath, req.query
                    )
                    .as_bytes()
                    .to_vec(),
                )
                .header("content-type", "text/html")
        }
    }
}

impl Route for HeaderRoute {
    fn path(&self) -> &'static str {
        "/header"
    }

    fn request_type(&self) -> RequestType {
        RequestType::Get
    }

    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| {
            Response::new()
                .body(format!("{:?}", _req.headers).as_bytes().to_vec())
                .header("content-type", "application/json")
        }
    }
}
