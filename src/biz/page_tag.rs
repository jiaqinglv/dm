use crate::data::page_tag;

pub struct PageTag {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct PageTagUsecase {
    pub repo: page_tag::PageTagRepo,
}

pub fn new_page_tag_usecase(repo: page_tag::PageTagRepo) -> PageTagUsecase {
    return PageTagUsecase { repo };
}
