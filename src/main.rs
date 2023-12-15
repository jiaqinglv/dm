#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
use std::{error::Error, sync::Arc};

use axum::Extension;
use tracing::info;

mod api;
mod biz;
mod config;
mod data;
mod routers;
mod server;
mod service;
mod wire;

use ezw_core::core as ezw;

use crate::{service::WebServices, wire::wire_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 配置读取加载
    let conf = ezw::load_config::<config::BootConfig>(ezw::config::ConfigType::YAML(
        "./config/default.yaml".to_string(),
    ))
    .await?;
    // 数据源加载
    let datas = data::new_data(&conf).await?;
    // 创建服务器实例
    let servers = server::new_servers(conf.clone()).await?;
    // 创建应用
    let app = ezw::new_app(conf.clone(), datas, servers, conf.id, conf.name, conf.version);

    // 创建 axum http 服务实例
    let app = app
        .init_log(tracing::Level::DEBUG)
        .expect("init log failed");

    // web服务层
    let services: Arc<WebServices> = Arc::new(wire_app(&app.conf, app.data.unwrap()));
    // 路由
    let router = routers::new_http_router().layer(Extension(services.clone()));
    let conf = app.conf;
    info!("server is running on http://{}:{}", conf.host, conf.port);

    app.servers.save(router).await?;

    Result::Ok(())
}
