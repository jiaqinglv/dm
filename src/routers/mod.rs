use axum::Router;
use tower_http::trace::TraceLayer;

use crate::api;

pub fn new_http_router() -> Router {
    let router = Router::new();
    // router.route("/favicon.ico", get());
    let api_router = api::register_http();
    router.merge(api_router).layer(TraceLayer::new_for_http())
}
