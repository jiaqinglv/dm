use tracing::error;

use super::Data;
use crate::{biz::block,  errors};

#[derive(Debug, Default)]
pub struct BlockRepo {
    pub data: Data,
}

pub fn new_block_repo(data: Data) -> BlockRepo {
    return BlockRepo { data };
}

impl BlockRepo {
    /// 创建块
    pub async fn create(&self, block: block::Block) -> Result<(), errors::WebError> {
        let rec = sqlx::query!(
            r#"
                INSERT INTO public.block (
                    block_id, block_name, parent_id, remark
                ) VALUES(
                    $1, $2, $3, $4
                );
            "#,
            block.block_id,
            block.block_name,
            block.parent_id,
            block.remark
        )
        .fetch_one(&self.data.pg.clone().unwrap())
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("添加失败:{}", err);
                return Err(errors::WebError::new(errors::WebErrorType::DataError, "添加失败，请重试"));
            }
        }
    }

    /// 删除块
    pub async fn delete(&self, block_id: i64) -> Result<(), errors::WebError> {
        let rec = sqlx::query!(
            r#"
                UPDATE public.block SET is_valid=false WHERE block_id = $1;
            "#,
            block_id,
        )
        .fetch_one(&self.data.pg.clone().unwrap())
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("添加失败:{}", err);
                return Err(errors::WebError::new(errors::WebErrorType::DataError, "删除失败，请重试"));
            }
        }
    }

    /// 更新块
    pub async fn update(&self, block: block::Block) -> Result<(), errors::WebError>  {
        let rec = sqlx::query!(
            r#"
            UPDATE public.block SET block_name=$1, parent_id=$2, update_at=$3, remark=$4 WHERE block_id=$5;
            "#,
            block.block_name,
            block.parent_id,
            block.update_at,
            block.remark,
            block.block_id,
        )
        .fetch_one(&self.data.pg.clone().unwrap())
        .await;

        match rec {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("添加失败:{}", err);
                return Err(errors::WebError::new(errors::WebErrorType::DataError, "添加失败，请重试"));
            }
        }
    }
}
