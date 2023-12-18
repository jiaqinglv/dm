use crate::data::model;

pub struct Model {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct ModelUsecase {
    pub repo: model::ModelRepo,
}

pub fn new_model_usecase(repo: model::ModelRepo) -> ModelUsecase {
    return ModelUsecase { repo };
}
