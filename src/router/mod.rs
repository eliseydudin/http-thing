mod receiver;
mod request;
mod response;

pub use receiver::Receiver;
pub use request::{Request, RequestType};
pub use response::Response;

pub trait Route {
    fn request_type(&self) -> RequestType;
    fn path(&self) -> &'static str;
    fn handler(&mut self) -> fn(Request) -> Response;
}

pub struct Router {
    routes: Vec<Box<dyn Route>>,
    default_handler: Option<Box<dyn Route>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: vec![],
            default_handler: None,
        }
    }

    pub fn add_route<R>(&mut self, route: R)
    where
        R: Route + 'static,
    {
        self.routes.push(Box::new(route));
    }

    pub fn add_default_handler<R>(&mut self, route: R)
    where
        R: Route + 'static,
    {
        self.default_handler = Some(Box::new(route));
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

        for route in &mut self.routes {
            if route.request_type() == rtype
                && (route.path() == path_s || path_s == format!("{}/", route.path()))
            {
                return Some(route.handler());
            }
        }

        if let Some(ref mut h) = self.default_handler {
            return Some(h.handler());
        }

        None
    }
}
