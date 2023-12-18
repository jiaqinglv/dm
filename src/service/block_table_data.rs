use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::block_table_data;

#[derive(Debug, Default)]
pub struct BlockTableDataService {
    pub name: &'static str,
    uc: block_table_data::BlockTableDataUsecase,
}

// 创建一个 BlockTableData 服务
pub fn new_block_table_data_service(
    name: &'static str,
    uc: block_table_data::BlockTableDataUsecase,
) -> BlockTableDataService {
    return BlockTableDataService { name, uc };
}

impl WebService for BlockTableDataService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl BlockTableDataService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
