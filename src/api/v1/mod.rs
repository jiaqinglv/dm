use axum::Router;
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

pub fn register_http() -> axum::Router {
    // 版本路由
    let router = Router::new();

    // v1版本 路由
    let v1 = Router::new()
        .nest("/hello", hello::register_http())
        .nest("/block", block::register_http());

    // 路由嵌套
    router.nest("/v1", v1)
}
