use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::page;

#[derive(Debug, Default)]
pub struct PageService {
    pub name: &'static str,
    uc: page::PageUsecase,
}

// 创建一个 Page 服务
pub fn new_page_service(name: &'static str, uc: page::PageUsecase) -> PageService {
    return PageService { name, uc };
}

impl WebService for PageService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl PageService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
