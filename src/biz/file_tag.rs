use crate::data::file_tag;

pub struct FileTag {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct FileTagUsecase {
    pub repo: file_tag::FileTagRepo,
}

pub fn new_file_tag_usecase(repo: file_tag::FileTagRepo) -> FileTagUsecase {
    return FileTagUsecase { repo };
}
