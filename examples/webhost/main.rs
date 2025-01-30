use clang_log::init;
use http_router::prelude::*;
use std::fs;


fn error404() -> Response {
    Response::new().body(include_bytes!("www/404/index.html")).header("content-type", "text/html")
}

fn load_file(path: &String) -> Response {
    Response::new().body(fs::read(path).unwrap_or(include_bytes!("www/404/index.html").to_vec()).as_slice())
}

struct RedirectRoute;

impl Route for RedirectRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/";

    fn handler(&mut self) -> fn(Request) -> Response {
        |req| {
            let path = format!("{}{}", fs::canonicalize("./examples/webhost/www/").unwrap_or(fs::canonicalize("./www/").unwrap_or(fs::canonicalize(".").expect("Wrongly configured server, directory www not found"))).display(), req.path);
            let exists = fs::exists(&path).unwrap_or(false);
            if exists {
                let metadata = fs::metadata(&path).expect("MetaData could not be reached");
                if metadata.is_dir() {
                    return load_file(&format!("{}{}", path, if path.ends_with("/") {"index.html"} else {"/index.html"})).header("content-type", "text/html");
                } else if metadata.is_file() {
                    return load_file(&path);
                } else {
                    return error404();
                }
            } else {
                error404()
            }
        } 
    }
}

fn main() {
    init(log::Level::Trace, "webhost");
    let mut server = Server::new(8080, 20);
    server.add_default_handler(RedirectRoute);
    server.run()
}
