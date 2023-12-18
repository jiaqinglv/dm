use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecResponse {
    pub status: bool,
    pub code: i32,
}
