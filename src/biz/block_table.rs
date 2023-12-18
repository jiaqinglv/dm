use crate::data::block_table;

pub struct BlockTable {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct BlockTableUsecase {
    pub repo: block_table::BlockTableRepo,
}

pub fn new_block_table_usecase(repo: block_table::BlockTableRepo) -> BlockTableUsecase {
    return BlockTableUsecase { repo };
}
