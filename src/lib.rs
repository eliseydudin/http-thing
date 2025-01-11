mod router;
mod server;
mod thread_pool;

pub mod prelude {
    pub use super::router::{Request, RequestType, Response, Route};
    pub use super::server::Server;
}
