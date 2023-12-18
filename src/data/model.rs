use super::Data;
use crate::biz::model;

#[derive(Debug, Default)]
pub struct ModelRepo {
    pub data: Data,
}

pub fn new_model_repo(data: Data) -> ModelRepo {
    return ModelRepo { data };
}

impl ModelRepo {
    #[allow(dead_code)]
    pub async fn create(&self, model: model::Model) -> model::Model {
        let name = model.name + " model";

        model::Model { name }
    }

    #[allow(dead_code)]
    pub fn create_many(&self) {}

    #[allow(dead_code)]
    pub fn list(&self) {}

    #[allow(dead_code)]
    pub fn info(&self) -> String {
        String::from("model world!")
    }

    #[allow(dead_code)]
    pub fn delete(&self) {}

    #[allow(dead_code)]
    pub fn update(&self) {}
}
