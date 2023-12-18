use serde::{Deserialize, Serialize};

/// 块
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    /// 块ID
    pub block_id: String,
    /// 块名称
    pub block_name: String,
    /// 父ID
    pub parent_id: String,
    /// 创建时间
    pub create_at: String,
    /// 更新时间
    pub update_at: String,
    /// 是否有效
    pub is_valid: bool,
    /// 备注
    pub remark: String,
}

/// 创建请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReqest {
    pub block_name: String,
    pub parent_id: String,
    pub remark: String,
}

/// 更新请求
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReqest {
    pub block_id: String,
    pub block_name: String,
    pub parent_id: String,
    pub remark: String,
}

/// 删除请求
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReqest {
    pub block_id: String,
}
