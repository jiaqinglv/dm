use crate::data::block_table_column;

pub struct BlockTableColumn {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct BlockTableColumnUsecase {
    pub repo: block_table_column::BlockTableColumnRepo,
}

pub fn new_block_table_column_usecase(
    repo: block_table_column::BlockTableColumnRepo,
) -> BlockTableColumnUsecase {
    return BlockTableColumnUsecase { repo };
}
