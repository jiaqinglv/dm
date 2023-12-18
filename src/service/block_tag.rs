use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::block_tag;

#[derive(Debug, Default)]
pub struct BlockTagService {
    pub name: &'static str,
    uc: block_tag::BlockTagUsecase,
}

// 创建一个 BlockTag 服务
pub fn new_block_tag_service(
    name: &'static str,
    uc: block_tag::BlockTagUsecase,
) -> BlockTagService {
    return BlockTagService { name, uc };
}

impl WebService for BlockTagService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl BlockTagService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
