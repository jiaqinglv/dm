use std::future::Future;

use axum::Json;

use super::WebService;
use crate::biz::file_tag;

#[derive(Debug, Default)]
pub struct FileTagService {
    pub name: &'static str,
    uc: file_tag::FileTagUsecase,
}

// 创建一个 FileTag 服务
pub fn new_file_tag_service(name: &'static str, uc: file_tag::FileTagUsecase) -> FileTagService {
    return FileTagService { name, uc };
}

impl WebService for FileTagService {
    // 获取姓名Future
    type NameFuture<'a> = impl Future<Output = &'static str> + 'a
    where
        Self: 'a;

    // 获取服务名称
    fn get_service_name(&self) -> Self::NameFuture<'_> {
        self.get_service_name()
    }
}

impl FileTagService {
    pub async fn get_service_name(&self) -> &'static str {
        return self.name;
    }
}
