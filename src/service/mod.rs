pub mod block;
pub mod block_table;
pub mod block_table_column;
pub mod block_table_data;
pub mod block_tag;
pub mod hello;

pub mod file;
pub mod file_tag;

pub mod model;

pub mod page;
pub mod page_tag;

pub mod tag;

use std::future::Future;

pub trait WebService {
    type NameFuture<'a>: Future<Output = &'static str> + Send + 'a
    where
        Self: 'a;
    fn get_service_name(&self) -> Self::NameFuture<'_>;
}

// web service集合
#[derive(Default)]
pub struct WebServices {
    pub hello_service: hello::HelloService,
    pub block_service: block::BlockService,
    pub block_table_service: block_table::BlockTableService,
    pub block_table_column_service: block_table_column::BlockTableColumnService,
    pub block_table_data_service: block_table_data::BlockTableDataService,
    pub block_tag_service: block_tag::BlockTagService,
    pub file_service: file::FileService,
    pub file_tag_service: file_tag::FileTagService,
    pub model_service: model::ModelService,
    pub page: page::PageService,
    pub page_tag: page_tag::PageTagService,
    pub tag: tag::TagService,
}

// 创建 web service集合
pub fn new_web_services(
    hello_service: hello::HelloService,
    block_service: block::BlockService,
    block_table_service: block_table::BlockTableService,
    block_table_column_service: block_table_column::BlockTableColumnService,
    block_table_data_service: block_table_data::BlockTableDataService,
    block_tag_service: block_tag::BlockTagService,
    file_service: file::FileService,
    file_tag_service: file_tag::FileTagService,
    model_service: model::ModelService,
    page: page::PageService,
    page_tag: page_tag::PageTagService,
    tag: tag::TagService,
) -> WebServices {
    WebServices {
        hello_service,
        block_service,
        block_table_service,
        block_table_column_service,
        block_table_data_service,
        block_tag_service,
        file_service,
        file_tag_service,
        model_service,
        page,
        page_tag,
        tag,
    }
}
