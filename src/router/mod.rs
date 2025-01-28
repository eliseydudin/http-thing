mod receiver;
mod request;
mod response;

use std::collections::HashMap;

pub use receiver::Receiver;
pub use request::{Request, RequestType};
pub use response::Response;

pub trait Route {
    const RTYPE: RequestType;
    const PATH: &'static str;

    fn handler(&mut self) -> fn(Request) -> Response;
}

type Handler = (RequestType, &'static str);

pub struct Router {
    routes: HashMap<Handler, fn(Request) -> Response>,
    default_handler: Option<fn(Request) -> Response>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default_handler: None,
        }
    }

    pub fn add_route<R>(&mut self, mut route: R)
    where
        R: Route + 'static,
    {
        let handler = (R::RTYPE, R::PATH);
        self.routes.insert(handler, route.handler());
    }

    pub fn add_default_handler<R>(&mut self, mut route: R)
    where
        R: Route + 'static,
    {
        self.default_handler = Some(route.handler());
    }

    pub fn find_handler<S>(
        &mut self,
        path: S,
        rtype: RequestType,
    ) -> Option<fn(Request) -> Response>
    where
        S: Into<String>,
    {
        let path_s: String = path.into();

        for (query, route) in &mut self.routes {
            if query.0 == rtype && query.1 == path_s {
                return Some(*route);
            }
        }

        if let Some(ref mut h) = self.default_handler {
            return Some(*h);
        }

        None
    }
}
