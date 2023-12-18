use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::block_table;

#[derive(Debug, Default)]
pub struct BlockTableService {
    pub name: &'static str,
    uc: block_table::BlockTableUsecase,
}

// 创建一个 BlockTable 服务
pub fn new_block_table_service(
    name: &'static str,
    uc: block_table::BlockTableUsecase,
) -> BlockTableService {
    return BlockTableService { name, uc };
}

impl WebService for BlockTableService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl BlockTableService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
