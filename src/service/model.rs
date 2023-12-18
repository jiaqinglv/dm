use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::model;

#[derive(Debug, Default)]
pub struct ModelService {
    pub name: &'static str,
    uc: model::ModelUsecase,
}

// 创建一个 Model 服务
pub fn new_model_service(name: &'static str, uc: model::ModelUsecase) -> ModelService {
    return ModelService { name, uc };
}

impl WebService for ModelService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl ModelService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
