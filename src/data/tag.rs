use super::Data;
use crate::biz::tag;

#[derive(Debug, Default)]
pub struct TagRepo {
    pub data: Data,
}

pub fn new_tag_repo(data: Data) -> TagRepo {
    return TagRepo { data };
}

impl TagRepo {
    #[allow(dead_code)]
    pub async fn create(&self, tag: tag::Tag) -> tag::Tag {
        let name = tag.name + " tag";

        tag::Tag { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("tag world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
