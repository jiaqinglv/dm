use crate::data::tag;

pub struct Tag {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct TagUsecase {
    pub repo: tag::TagRepo,
}

pub fn new_tag_usecase(repo: tag::TagRepo) -> TagUsecase {
    return TagUsecase { repo };
}
