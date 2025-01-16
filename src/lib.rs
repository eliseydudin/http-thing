#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::question_mark_used,
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,
    clippy::min_ident_chars,
    clippy::std_instead_of_alloc,
    clippy::error_impl_error,
    clippy::std_instead_of_core,
    clippy::string_slice,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::indexing_slicing,
    clippy::expect_used,
    clippy::mod_module_files,
    clippy::pub_use,
    clippy::cargo_common_metadata,
    reason = "These warns just complicate the code"
)]

mod router;
mod server;
mod thread_pool;

pub mod prelude {
    pub use super::router::{Request, RequestType, Response, Route};
    pub use super::server::Server;
}
