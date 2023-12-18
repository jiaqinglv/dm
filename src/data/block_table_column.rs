use super::Data;
use crate::biz::block_table_column;

#[derive(Debug, Default)]
pub struct BlockTableColumnRepo {
    pub data: Data,
}

pub fn new_block_table_column_repo(data: Data) -> BlockTableColumnRepo {
    return BlockTableColumnRepo { data };
}

impl BlockTableColumnRepo {
    #[allow(dead_code)]
    pub async fn create(
        &self,
        block_table_column: block_table_column::BlockTableColumn,
    ) -> block_table_column::BlockTableColumn {
        let name = block_table_column.name + " block_table_column";

        block_table_column::BlockTableColumn { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("block_table_column world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
