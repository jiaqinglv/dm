use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::file;

#[derive(Debug, Default)]
pub struct FileService {
    pub name: &'static str,
    uc: file::FileUsecase,
}

// 创建一个 File 服务
pub fn new_file_service(name: &'static str, uc: file::FileUsecase) -> FileService {
    return FileService { name, uc };
}

impl WebService for FileService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl FileService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
