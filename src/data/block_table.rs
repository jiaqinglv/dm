use super::Data;
use crate::biz::block_table;

#[derive(Debug, Default)]
pub struct BlockTableRepo {
    pub data: Data,
}

pub fn new_block_table_repo(data: Data) -> BlockTableRepo {
    return BlockTableRepo { data };
}

impl BlockTableRepo {
    #[allow(dead_code)]
    pub async fn create(&self, block_table: block_table::BlockTable) -> block_table::BlockTable {
        let name = block_table.name + " block_table";

        block_table::BlockTable { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("block_table world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
