use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::TimeZone;

use crate::data::block;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// 块ID
    pub block_id: i64,
    /// 块名称
    pub block_name: String,
    /// 父ID
    pub parent_id: i64,
    /// 创建时间
    pub create_at: String,
    /// 更新时间
    pub update_at: DateTime<chrono::Local>,
    /// 是否有效
    pub is_valid: bool,
    /// 备注
    pub remark: String,
}

#[derive(Debug, Default)]
pub struct BlockUsecase {
    pub repo: block::BlockRepo,
}

pub fn new_block_usecase(repo: block::BlockRepo) -> BlockUsecase {
    return BlockUsecase { repo };
}
