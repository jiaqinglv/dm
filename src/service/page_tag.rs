use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::page_tag;

#[derive(Debug, Default)]
pub struct PageTagService {
    pub name: &'static str,
    uc: page_tag::PageTagUsecase,
}

// 创建一个 PageTag 服务
pub fn new_page_tag_service(name: &'static str, uc: page_tag::PageTagUsecase) -> PageTagService {
    return PageTagService { name, uc };
}

impl WebService for PageTagService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl PageTagService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
