use super::Data;
use crate::biz::file;

#[derive(Debug, Default)]
pub struct FileRepo {
    pub data: Data,
}

pub fn new_file_repo(data: Data) -> FileRepo {
    return FileRepo { data };
}

impl FileRepo {
    #[allow(dead_code)]
    pub async fn create(&self, file: file::File) -> file::File {
        let name = file.name + " file";

        file::File { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("file world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
