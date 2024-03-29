use ezw_core::core::data::DataSource;

use crate::config;

pub mod hello;

pub mod block;
pub mod block_table;
pub mod block_table_column;
pub mod block_table_data;
pub mod block_tag;

pub mod file;
pub mod file_tag;

pub mod model;

pub mod page;
pub mod page_tag;

pub mod tag;

use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[derive(Debug, Clone)]
pub struct Data {
    pub pg: Option<sqlx::Pool<sqlx::Postgres>>,
}

impl DataSource for Data {}

impl Default for Data {
    fn default() -> Self {
        Self { pg: None }
    }
}

#[derive(sqlx::FromRow, Debug)]
struct TimeNow {
    pub now: chrono::DateTime<chrono::Local>,
}

pub async fn new_data(conf: &config::BootConfig) -> Result<Data, Box<dyn std::error::Error>> {
    println!("连接数据库中...");

    let mut d = Data { pg: None };

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conf.pg.connect_str)
        .await?;

    d.pg = Some(pool);

    let time = sqlx::query_as::<_, TimeNow>("SELECT now()")
        .fetch_one(&d.pg.clone().unwrap())
        .await?;

    println!("{} INIT 连接数据库连接成功[OK]  ", time.now);

    return Ok(d);
}
