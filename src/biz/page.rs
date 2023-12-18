use crate::data::page;

pub struct Page {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct PageUsecase {
    pub repo: page::PageRepo,
}

pub fn new_page_usecase(repo: page::PageRepo) -> PageUsecase {
    return PageUsecase { repo };
}
