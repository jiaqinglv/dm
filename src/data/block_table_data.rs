use super::Data;
use crate::biz::block_table_data;

#[derive(Debug, Default)]
pub struct BlockTableDataRepo {
    pub data: Data,
}

pub fn new_block_table_data_repo(data: Data) -> BlockTableDataRepo {
    return BlockTableDataRepo { data };
}

impl BlockTableDataRepo {
    #[allow(dead_code)]
    pub async fn create(
        &self,
        block_table_data: block_table_data::BlockTableData,
    ) -> block_table_data::BlockTableData {
        let name = block_table_data.name + " block_table_data";

        block_table_data::BlockTableData { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("block_table_data world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
