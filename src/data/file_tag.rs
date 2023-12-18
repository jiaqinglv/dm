use super::Data;
use crate::biz::file_tag;

#[derive(Debug, Default)]
pub struct FileTagRepo {
    pub data: Data,
}

pub fn new_file_tag_repo(data: Data) -> FileTagRepo {
    return FileTagRepo { data };
}

impl FileTagRepo {
    #[allow(dead_code)]
    pub async fn create(&self, file_tag: file_tag::FileTag) -> file_tag::FileTag {
        let name = file_tag.name + " file_tag";

        file_tag::FileTag { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("file_tag world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
