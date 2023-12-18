use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::block_table_column;

#[derive(Debug, Default)]
pub struct BlockTableColumnService {
    pub name: &'static str,
    uc: block_table_column::BlockTableColumnUsecase,
}

// 创建一个 BlockTableColumn 服务
pub fn new_block_table_column_service(
    name: &'static str,
    uc: block_table_column::BlockTableColumnUsecase,
) -> BlockTableColumnService {
    return BlockTableColumnService { name, uc };
}

impl WebService for BlockTableColumnService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl BlockTableColumnService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
