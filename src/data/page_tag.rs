use super::Data;
use crate::biz::page_tag;

#[derive(Debug, Default)]
pub struct PageTagRepo {
    pub data: Data,
}

pub fn new_page_tag_repo(data: Data) -> PageTagRepo {
    return PageTagRepo { data };
}

impl PageTagRepo {
    #[allow(dead_code)]
    pub async fn create(&self, page_tag: page_tag::PageTag) -> page_tag::PageTag {
        let name = page_tag.name + " page_tag";

        page_tag::PageTag { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("page_tag world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
