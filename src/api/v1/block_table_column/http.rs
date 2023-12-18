use std::sync::Arc;

use axum::{extract::Path, response::Json, Extension};
use hyper::StatusCode;

use crate::{api::v1::hello::response, service::WebServices};

pub async fn get_hello(
    Path(name): Path<String>,
    Extension(services): Extension<Arc<WebServices>>,
) -> (StatusCode, Json<response::HelloResponse>) {
    (StatusCode::OK, services.hello_service.create(name).await)
}
