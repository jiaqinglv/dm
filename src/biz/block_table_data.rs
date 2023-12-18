use crate::data::block_table_data;

pub struct BlockTableData {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct BlockTableDataUsecase {
    pub repo: block_table_data::BlockTableDataRepo,
}

pub fn new_block_table_data_usecase(
    repo: block_table_data::BlockTableDataRepo,
) -> BlockTableDataUsecase {
    return BlockTableDataUsecase { repo };
}
