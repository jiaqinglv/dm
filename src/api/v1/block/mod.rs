use axum::{
    routing::{delete, post},
    Router,
};

pub mod http;
pub mod requset;
pub mod response;

pub fn register_http() -> Router {
    let r = Router::new();
    r.route("/create", post(http::create))
        .route("/update", post(http::update))
        .route("/:block_id", delete(http::delete))
}
