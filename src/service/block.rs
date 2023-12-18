use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::block;

#[derive(Debug, Default)]
pub struct BlockService {
    pub name: &'static str,
    uc: block::BlockUsecase,
}

// 创建一个 Block 服务
pub fn new_block_service(name: &'static str, uc: block::BlockUsecase) -> BlockService {
    return BlockService { name, uc };
}

impl WebService for BlockService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl BlockService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
