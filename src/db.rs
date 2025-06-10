use iced::Task;

use crate::model::Model;

pub async fn load_erd_model(_schema: String) -> Result<Model, String> {
    todo!()
}

pub async fn sync_erd_model(_tables: Vec<super::model::Table>) -> Result<(), String> {
    todo!()
}
