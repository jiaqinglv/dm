use axum::{routing::get, Router};

pub mod http;
pub mod requset;
pub mod response;

pub fn register_http() -> Router {
    let r = Router::new();
    r.route("/:name", get(http::get_hello))
}
