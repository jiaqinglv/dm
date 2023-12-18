use crate::data::file;

pub struct File {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct FileUsecase {
    pub repo: file::FileRepo,
}

pub fn new_file_usecase(repo: file::FileRepo) -> FileUsecase {
    return FileUsecase { repo };
}
