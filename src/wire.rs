use crate::{
    biz::{
        block::new_block_usecase, block_table::new_block_table_usecase,
        block_table_column::new_block_table_column_usecase,
        block_table_data::new_block_table_data_usecase, block_tag::new_block_tag_usecase,
        file::new_file_usecase, file_tag::new_file_tag_usecase, hello::new_hello_usecase,
        model::new_model_usecase, page::new_page_usecase, page_tag::new_page_tag_usecase,
        tag::new_tag_usecase,
    },
    config::BootConfig,
    data::{
        block::new_block_repo, block_table::new_block_table_repo,
        block_table_column::new_block_table_column_repo,
        block_table_data::new_block_table_data_repo, block_tag::new_block_tag_repo,
        file::new_file_repo, file_tag::new_file_tag_repo, hello::new_hello_repo,
        model::new_model_repo, page::new_page_repo, page_tag::new_page_tag_repo, tag::new_tag_repo,
        Data,
    },
    service::{
        block::new_block_service, block_table::new_block_table_service,
        block_table_column::new_block_table_column_service,
        block_table_data::new_block_table_data_service, block_tag::new_block_tag_service,
        file::new_file_service, file_tag::new_file_tag_service, hello::new_hello_service,
        model::new_model_service, new_web_services, page::new_page_service,
        page_tag::new_page_tag_service, tag::new_tag_service, WebServices,
    },
};

pub fn wire_app(_conf: &BootConfig, data: Data) -> WebServices {
    // hello
    let hello_repo = new_hello_repo(data.clone());
    let hello_usecase = new_hello_usecase(hello_repo);
    let hello_service = new_hello_service("hello_service", hello_usecase);

    // block
    let block_repo = new_block_repo(data.clone());
    let block_usecase = new_block_usecase(block_repo);
    let block_service = new_block_service("block_service", block_usecase);

    // block_table
    let block_table_repo = new_block_table_repo(data.clone());
    let block_table_usecase = new_block_table_usecase(block_table_repo);
    let block_table_service = new_block_table_service("block_table_service", block_table_usecase);

    // block_table_column
    let block_table_column_repo = new_block_table_column_repo(data.clone());
    let block_table_column_usecase = new_block_table_column_usecase(block_table_column_repo);
    let block_table_column_service =
        new_block_table_column_service("block_table_column_service", block_table_column_usecase);

    // block_table_data
    let block_table_data_repo = new_block_table_data_repo(data.clone());
    let block_table_data_usecase = new_block_table_data_usecase(block_table_data_repo);
    let block_table_data_service =
        new_block_table_data_service("block_table_data_service", block_table_data_usecase);

    // block_tag
    let block_tag_repo = new_block_tag_repo(data.clone());
    let block_tag_usecase = new_block_tag_usecase(block_tag_repo);
    let block_tag_service = new_block_tag_service("block_tag_service", block_tag_usecase);

    // file
    let file_repo = new_file_repo(data.clone());
    let file_usecase = new_file_usecase(file_repo);
    let file_service = new_file_service("file_service", file_usecase);

    // file_tag
    let file_tag_repo = new_file_tag_repo(data.clone());
    let file_tag_usecase = new_file_tag_usecase(file_tag_repo);
    let file_tag_service = new_file_tag_service("file_tag_service", file_tag_usecase);

    // model
    let model_repo = new_model_repo(data.clone());
    let model_usecase = new_model_usecase(model_repo);
    let model_service = new_model_service("model_service", model_usecase);

    // page
    let page_repo = new_page_repo(data.clone());
    let page_usecase = new_page_usecase(page_repo);
    let page_service = new_page_service("page_service", page_usecase);

    // page_tag
    let page_tag_repo = new_page_tag_repo(data.clone());
    let page_tag_usecase = new_page_tag_usecase(page_tag_repo);
    let page_tag_service = new_page_tag_service("page_tag_service", page_tag_usecase);

    // tag
    let tag_repo = new_tag_repo(data.clone());
    let tag_usecase = new_tag_usecase(tag_repo);
    let tag_service = new_tag_service("tag_service", tag_usecase);

    new_web_services(
        hello_service,
        block_service,
        block_table_service,
        block_table_column_service,
        block_table_data_service,
        block_tag_service,
        file_service,
        file_tag_service,
        model_service,
        page_service,
        page_tag_service,
        tag_service,
    )
}
