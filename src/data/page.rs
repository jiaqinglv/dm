use super::Data;
use crate::biz::page;

#[derive(Debug, Default)]
pub struct PageRepo {
    pub data: Data,
}

pub fn new_page_repo(data: Data) -> PageRepo {
    return PageRepo { data };
}

impl PageRepo {
    #[allow(dead_code)]
    pub async fn create(&self, page: page::Page) -> page::Page {
        let name = page.name + " page";

        page::Page { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("page world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
