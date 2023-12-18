use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::tag;

#[derive(Debug, Default)]
pub struct TagService {
    pub name: &'static str,
    uc: tag::TagUsecase,
}

// 创建一个 Tag 服务
pub fn new_tag_service(name: &'static str, uc: tag::TagUsecase) -> TagService {
    return TagService { name, uc };
}

impl WebService for TagService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl TagService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
