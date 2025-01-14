use http_thing::prelude::*;

pub struct PingPongRoute;
pub struct RatRoute;
pub struct LostRoute;
pub struct HeaderRoute;
pub struct SongSubRoute;

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
                .body(b"<!DOCTYPE html>
                <html><head><title>Hello</title></head>
                <body><img src=\"https://dempah.com/bigsearch/silly/media/image/animal/rat/norwayrat-001-reverse.jpg\"></body></html>".to_vec())
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
                        "<!DOCTYPE html>
                <html><head><title>404</title></head>
                <body><h1>404</h1><ul>
                <li>Path: {:#?}</li>
                <li>Address: {:#?}</li>
                <li>Data: {:#?}</li>
                <li>Headers: {:#?}</li>
                <li>Fullpath: {:#?}</li>
                <li>Query: {:#?}</li>
                </ul></body></html>",
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

impl Route for SongSubRoute {
    fn path(&self) -> &'static str {
        "/song"
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
