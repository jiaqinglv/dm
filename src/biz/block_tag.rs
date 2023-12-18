use crate::data::block_tag;

pub struct BlockTag {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct BlockTagUsecase {
    pub repo: block_tag::BlockTagRepo,
}

pub fn new_block_tag_usecase(repo: block_tag::BlockTagRepo) -> BlockTagUsecase {
    return BlockTagUsecase { repo };
}
