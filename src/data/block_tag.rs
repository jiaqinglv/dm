use super::Data;
use crate::biz::block_tag;

#[derive(Debug, Default)]
pub struct BlockTagRepo {
    pub data: Data,
}

pub fn new_block_tag_repo(data: Data) -> BlockTagRepo {
    return BlockTagRepo { data };
}

impl BlockTagRepo {
    #[allow(dead_code)]
    pub async fn create(&self, block_tag: block_tag::BlockTag) -> block_tag::BlockTag {
        let name = block_tag.name + " block_tag";

        block_tag::BlockTag { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("block_tag world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
