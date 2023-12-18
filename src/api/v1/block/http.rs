use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};

use super::{requset, response};
use crate::service::WebServices;

pub async fn create(
    Extension(services): Extension<Arc<WebServices>>,
    Json(payload): Json<requset::CreateReqest>,
) -> (StatusCode, Json<response::ExecResponse>) {
    let res = response::ExecResponse {
        status: true,
        code: 0,
    };
    (StatusCode::OK, Json(res))
}

pub async fn update(
    Extension(services): Extension<Arc<WebServices>>,
    Json(payload): Json<requset::UpdateReqest>,
) -> (StatusCode, Json<response::ExecResponse>) {
    let res = response::ExecResponse {
        status: true,
        code: 0,
    };
    (StatusCode::OK, Json(res))
}

pub async fn delete(
    Path(block_id): Path<String>,
    Extension(services): Extension<Arc<WebServices>>,
) -> (StatusCode, Json<response::ExecResponse>) {
    let res = response::ExecResponse {
        status: true,
        code: 0,
    };

    (StatusCode::OK, Json(res))
}
