mod request;
mod response;

pub use request::{Request, RequestType};
pub use response::{Response, ResponseBuilder};

// pub trait Route {
//     fn request_type(&self) -> RequestType;
//     fn path(&self) -> &'static str;

//     fn handle(&mut self, req: &Request) -> Response;
// }
